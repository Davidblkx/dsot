import {
    type ColumnDef,
    type CreateTable,
    parseSQL,
    // deno-lint-ignore no-unversioned-import
} from "npm:@danscan/sqlite3-parser";

function readFlag(flag: string[]): boolean {
    return Deno.args.some((arg) => flag.includes(arg));
}

function readFlagValue<T>(
    flag: string[],
    defaultValue: T,
    map?: (e: string) => T,
): T {
    for (const f of flag) {
        const lookup = `${f}=`;
        const value = Deno.args.find((arg) => arg.startsWith(lookup));
        if (value) {
            return map ? map(value) : (value as unknown as T);
        }
    }
    return defaultValue;
}

// ----------------
const isOverwrite = readFlag(["--force", "-f"]);
const migrations_folder = readFlagValue(
    ["--migrations-folder", "-m"],
    "./migrations",
    (e) => e.replace(/\/+$/, ""),
);
const isListTables = readFlag(["--list-tables", "-l"]);
const table_names = Deno.args.filter((arg) => !arg.startsWith("-"));

if (Deno.args.includes("--help") || Deno.args.includes("-h")) {
    console.log(
        `Usage: deno run -A scripts/generate_sql_fts5.ts [options] [table_names...]

Options:
  -f, --force                   Overwrite existing migrations
  -m, --migrations-folder=PATH  Specify the migrations folder (default: ./migrations)
  -l, --list-tables             List all tables found in current migrations and exit
  -h, --help                    Show this help message and exit

If table_names are provided, only generate migrations for those tables.`,
    );
    Deno.exit(0);
}
// ----------------

type TableInfo = {
    name: string;
    columns: string[];
    id: string;
};

const unsupported_column_types: string[] = [
    "Blob",
    "Binary",
    "Boolean",
    "Varbinary",
    "Binary",
    "Custom",
];

type Migration = {
    path: string;
    name: string;
};

function hasObjectKey<T extends string, TValue = unknown>(
    obj: unknown,
    key: T,
): obj is { [K in T]: TValue } {
    return typeof obj === "object" && obj !== null && key in obj;
}

function readIdent(ident: unknown): string {
    if (hasObjectKey(ident, "Identifier")) {
        if (hasObjectKey(ident.Identifier, "value")) {
            return ident.Identifier.value as string;
        }
    } else if (hasObjectKey(ident, "value")) {
        return ident.value as string;
    }
    throw new Error("Invalid identifier: " + JSON.stringify(ident));
}

function isSupportedColumnType(type: unknown): boolean {
    for (const unsupported_type of unsupported_column_types) {
        if (hasObjectKey(type, unsupported_type)) {
            return false;
        }
    }
    return true;
}

function readPrimaryKey(columns: readonly ColumnDef[]): string {
    for (const column of columns) {
        if (
            hasObjectKey<"options", { "option": unknown }[]>(column, "options")
        ) {
            for (const p of column.options) {
                if (
                    hasObjectKey<"Unique", { "is_primary": boolean }>(
                        p.option,
                        "Unique",
                    )
                ) {
                    if (p.option.Unique.is_primary) {
                        return readIdent(column.name);
                    }
                }
            }
        }
    }

    throw new Error(
        "No primary key found in table: " + JSON.stringify(columns[0], null, 2),
    );
}

function readTableInfo(table: CreateTable): TableInfo {
    const name = readIdent(table.name[0]);
    const columns: string[] = table.columns.filter((c) =>
        isSupportedColumnType(c.data_type)
    ).map((c) => readIdent(c.name));
    const id = readPrimaryKey(table.columns);

    return {
        name,
        columns,
        id,
    };
}

async function readSqlFile(path: string): Promise<TableInfo[]> {
    const sqlContent = await Deno.readTextFile(path);
    const sql = await parseSQL(sqlContent);
    return sql
        .filter((e) => "CreateTable" in e)
        .map((e) => readTableInfo(e.CreateTable));
}

async function readCurrentMigrations(): Promise<Migration[]> {
    const migrations: Migration[] = [];
    for await (const dirEntry of Deno.readDir(migrations_folder)) {
        if (dirEntry.isFile && dirEntry.name.endsWith(".sql")) {
            migrations.push({
                path: `${migrations_folder}/${dirEntry.name}`,
                name: dirEntry.name.replace(/^\d+_/, "").replace(/\.sql$/, ""),
            });
        }
    }
    return migrations;
}

async function generateTableSql(
    table: TableInfo,
    tick_minute: number,
    tick_second: number,
): Promise<void> {
    const ftsTableName = `${table.name}_fts`;
    // Format migration ID as YYYYMMDDHHMMSS
    const migration_id = new Date().toISOString()
        .replace(/[-:T]/g, "")
        .slice(0, 10) +
        String(tick_minute).padStart(2, "0") +
        String(tick_second).padStart(2, "0");

    const migrationFile =
        `${migrations_folder}/${migration_id}_${ftsTableName}.sql`;
    const content = `
-- Generated migration for FTS5 table ${ftsTableName}
CREATE VIRTUAL TABLE ${ftsTableName} USING fts5 (
    ${table.id} UNINDEXED,
    ${table.columns.map((col) => `"${col}"`).join(",\n    ")},
);

-- Triggers to keep the FTS5 table in sync
CREATE TRIGGER ${table.name}_ai AFTER INSERT ON ${table.name} BEGIN
    INSERT INTO ${ftsTableName} (${table.id}, ${
        table.columns.map((col) => `"${col}"`).join(", ")
    })
    VALUES (new.${table.id}, ${
        table.columns.map((col) => `new."${col}"`).join(", ")
    });
END;

CREATE TRIGGER ${table.name}_ad AFTER DELETE ON ${table.name} BEGIN
    INSERT INTO ${ftsTableName} (${ftsTableName}, ${table.id}, ${
        table.columns.map((col) => `"${col}"`).join(", ")
    })
    VALUES ('delete', old.${table.id}, ${
        table.columns.map((col) => `old."${col}"`).join(", ")
    });
END;

CREATE TRIGGER ${table.name}_au AFTER UPDATE ON ${table.name} BEGIN
    INSERT INTO ${ftsTableName} (${ftsTableName}, ${table.id}, ${
        table.columns.map((col) => `"${col}"`).join(", ")
    })
    VALUES ('delete', old.${table.id}, ${
        table.columns.map((col) => `old."${col}"`).join(", ")
    });
    INSERT INTO ${ftsTableName} (${table.id}, ${
        table.columns.map((col) => `"${col}"`).join(", ")
    })
    VALUES (new.${table.id}, ${
        table.columns.map((col) => `new."${col}"`).join(", ")
    });
END;

-- Initial population of the FTS5 table
INSERT INTO ${ftsTableName} (${table.id}, ${
        table.columns.map((col) => `"${col}"`).join(", ")
    })
SELECT ${table.id}, ${
        table.columns.map((col) => `"${col}"`).join(", ")
    } FROM ${table.name};
`;

    await Deno.writeTextFile(migrationFile, content.trimStart());
    console.log(
        `Generated migration for table ${table.name}: ${migrationFile}`,
    );
}

const current_migrations = await readCurrentMigrations();
const tables: TableInfo[] = [];
for (const migration of current_migrations) {
    if (migration.name.endsWith("_fts")) {
        // skip existing fts migrations
        continue;
    }

    const file_tables = await readSqlFile(migration.path);
    tables.push(...file_tables);
}

if (isListTables) {
    const toList = tables.map((t) => t.name).sort();
    console.log(toList.join("\n"));
    Deno.exit(0);
}

const toGenerate = table_names.length > 0
    ? tables.filter((t) => table_names.includes(t.name))
    : tables;

let tick = 0;
const now = new Date();
for (const table of toGenerate) {
    tick += 1;
    let old_migration: string | undefined;
    if (current_migrations.some((m) => m.name === `${table.name}_fts`)) {
        if (!isOverwrite) {
            console.log(
                `Skipping table ${table.name}, migration already exists. Use --force to overwrite.`,
            );
            continue;
        }

        console.log(
            `Overwriting existing migration for table ${table.name} due to --force flag.`,
        );
        old_migration = current_migrations.find((m) =>
            m.name === `${table.name}_fts`
        )?.path;
    }

    // add tick as seconds to current time to ensure unique migration IDs
    const future = new Date(now.getTime() + tick * 1000);
    const tick_minute = future.getMinutes();
    const tick_second = future.getSeconds();
    await generateTableSql(table, tick_minute, tick_second);
}
