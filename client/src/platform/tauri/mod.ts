import type {
    CommandInput,
    CommandName,
    CommandOutput,
    CommandResult,
    Platform,
} from "../typings.d.ts";

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
    executeCommand: function <N extends CommandName>(
        name: N,
        input: CommandInput[N],
    ): CommandResult<CommandOutput[N]> {
        throw new Error("Function not implemented.");
    },
};
