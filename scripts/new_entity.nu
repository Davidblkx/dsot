# Script to create the file structure for a new entity

def confirm [message: string] {
    print $"($message) \(y/N\)? "
    let response = (input -s -n 1)
    if $response == "y" {
        return true
    } else {
        return false
    }
}

def create_file [
    file_path: string,
    content: string
] {
    mut create_file = not ($file_path | path exists);
    if not $create_file {
        print $"File already exists: ($file_path)"
        if (confirm "Overwrite file?") {
            $create_file = true
        }
    }

    if $create_file {
        echo $content | str trim | save -f $file_path
    }
}

def main [script_name: string] {
    let name = (if $script_name == "" {
        print "Enter the name of the entity: "
        (input)
    } else {
        $script_name
    })

    print $"Creating entity: ($name)"
    let entName = ($name | str camel-case | str capitalize)

    # Create root folder
    let root = $"core/src/model/entities/($name)";
    mkdir $root

    # Create entity file
    let file = $"core/src/model/entities/($name)/entity.rs";
    print $"Creating entity file: ($file)"
    create_file $file $"use uuid::Uuid;

#[derive\(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow\)]
pub struct ($entName)V0 {
    pub id: Uuid,
    pub mbid: Option<Uuid>
}

crate::dsot_storage_declare_model!\(
    ($entName) {
        0: ($entName)V0
    }
    \"
    Documentation here
    \"
\);
    "

    # Create impl file
    let impl_file = $"core/src/model/entities/($name)/impls.rs";
    print $"Creating entity impl file: ($impl_file)"
    create_file $impl_file $"use super::($entName);
impl ($entName) {
}
"

    # Create update op file
    let update_op_file = $"core/src/model/entities/($name)/op.rs";
    print $"Creating update op file: ($update_op_file)"
    create_file $update_op_file $"use uuid::Uuid;

#[derive\(Debug, Clone, serde::Serialize, serde::Deserialize\)]
pub enum ($entName)UpdateOpV0 {
    SetMbid\(Option<Uuid>\),
}

crate::dsot_storage_declare_model!\(
($entName)UpdateOp {
    0: ($entName)UpdateOpV0
} \"Represents an operation to update a ($name) in the database.\"
\);
"

    # Create sql file
    let sql_file = $"core/src/model/entities/($name)/sql.rs";
    print $"Creating sql file: ($sql_file)"
    create_file $sql_file $"use super::{($entName), op::($entName)UpdateOp};

crate::dsot_sql_entity!\([\"($name)\"] ($entName) with ($entName)UpdateOp {
    mbid
}\);

#[cfg\(test\)]
mod tests {
    use super::*;

    #[sqlx::test\(migrations = \"../migrations\"\)]
    async fn can_query\(pool: sqlx::SqlitePool\) {
        let trx = pool.begin\(\).await.unwrap\(\);
    }
}
"

    # Create mod file
    let mod_file = $"core/src/model/entities/($name)/mod.rs";
    print $"Creating mod file: ($mod_file)"
    create_file $mod_file $"mod entity;
mod impls;

pub mod op;
pub mod sql;

pub use entity::*;
";

    print $"Entity ($name) created successfully!"
}
