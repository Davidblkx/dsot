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
};

export type CommandOutput = {
    "users-list": {
        users: string[];
    };
};

export type CommandName = keyof CommandInput & keyof CommandOutput;

export type CommandHandler = {
    [K in CommandName]: (
        input: CommandInput[K],
    ) => CommandResult<CommandOutput[K]>;
};
