import { User } from "./model/user.ts";
import { Inbox } from "./model/inbox.ts";

export type CommandResult<T> = Promise<
    {
        success: true;
        value: T;
    } | {
        success: false;
        error: string;
    }
>;

export type CommandInput = {
    "users-list": Record<string, never>;
    "inbox-list": { limit: number; offset: number };
    "inbox-create": Partial<Inbox>;
    "inbox-update": Inbox;
    "inbox-delete": { id: string };
};

export type CommandOutput = {
    "users-list": User[];
    "inbox-list": Inbox[];
    "inbox-create": string;
    "inbox-update": boolean;
    "inbox-delete": boolean;
};

export type CommandName = keyof CommandInput & keyof CommandOutput;

export type CommandHandler = {
    [K in CommandName]: (
        input: CommandInput[K],
    ) => CommandResult<CommandOutput[K]>;
};

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
