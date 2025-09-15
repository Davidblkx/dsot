import {
    CommandInput,
    CommandName,
    CommandOutput,
    CommandResult,
} from "./types/commands.d.ts";

export * from "./types/commands.d.ts";

export interface Platform {
    setCurrentUserId(id: string | undefined): void;
    getCurrentUserId(): string | undefined;
    getRuntimeName(): string;
    executeCommand: <N extends CommandName>(
        name: N,
        input: CommandInput[N],
    ) => CommandResult<CommandOutput[N]>;
}

declare module "$platform" {
    const core: Platform;
}
