import type {
    CommandInput,
    CommandName,
    CommandOutput,
    CommandResult,
    Platform,
} from "../typings.d.ts";
import { invoke } from "@tauri-apps/api/core";

let CURRENT_USER_ID: string | undefined = undefined;

export const core: Platform = {
    getRuntimeName(): string {
        return "tauri";
    },
    setCurrentUserId: function (id: string | undefined): void {
        CURRENT_USER_ID = id;
    },
    getCurrentUserId: function (): string | undefined {
        return CURRENT_USER_ID;
    },
    executeCommand: async function <N extends CommandName>(
        name: N,
        input: CommandInput[N],
    ): CommandResult<CommandOutput[N]> {
        const invokeName = name.replace(/-/g, "_");

        const paramName = commandParams[name];
        const invokeArgs = paramName === 0 ? undefined : { [paramName]: input };
        const headers = CURRENT_USER_ID
            ? {
                headers: { "X-User-Id": CURRENT_USER_ID },
            }
            : undefined;

        try {
            return await invoke(invokeName, invokeArgs, headers);
        } catch (err) {
            const errMessage = err instanceof Error ? err.message : String(err);
            console.error(
                `Error executing command "${name}" via Tauri invoke "${invokeName}": ${errMessage}`,
            );
            return {
                success: false,
                error: `Error executing command: ${errMessage}`,
            };
        }
    },
};

type CommandParams = {
    [K in CommandName]: 'input' | 0;
};

const commandParams: CommandParams = {
    "users-list": 0,
    "inbox-list": "input",
    "inbox-create": "input",
    "inbox-update": "input",
    "inbox-delete": "input",
};
