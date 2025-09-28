import type {
    CommandInput,
    CommandName,
    CommandOutput,
    CommandResult,
    Platform,
} from "../typings.d.ts";

const DEFAULT_VERSION = "v1";
let CURRENT_USER_ID: string | undefined;

export const core: Platform = {
    getRuntimeName(): string {
        return "web";
    },
    executeCommand: async function <N extends CommandName>(
        name: N,
        input: CommandInput[N],
    ): CommandResult<CommandOutput[N]> {
        const endpoint = commandEndpoints[name];

        // deno-lint-ignore no-explicit-any
        const reqInput = JSON.stringify(input) as any;
        const url = buildURL(endpoint);
        const headers = new Headers();
        headers.append("Content-Type", "application/json");
        if (CURRENT_USER_ID) {
            headers.append("X-User-Id", CURRENT_USER_ID);
        }

        const request = new Request(url, {
            method: "POST",
            headers,
            body: reqInput,
        });

        try {
            const response = await fetch(request);
            if (!response.ok) {
                const errText = await response.text();
                console.error(
                    `Error response for command "${name}" at "${url}": ${response.status} ${response.statusText} - ${errText}`,
                );
                return {
                    success: false,
                    error:
                        `Server responded with status ${response.status}: ${response.statusText}`,
                };
            }

            return {
                success: true,
                value: await response.json(),
            };
        } catch (err) {
            const errMessage = err instanceof Error ? err.message : String(err);
            console.error(
                `Error executing command "${name}" at "${url}": ${errMessage}`,
            );

            return {
                success: false,
                error: errMessage,
            };
        }
    },

    setCurrentUserId: function (id: string | undefined): void {
        CURRENT_USER_ID = id;
    },
    getCurrentUserId: function (): string | undefined {
        return CURRENT_USER_ID;
    },
};

type CommandEndpoint = {
    [K in CommandName]: string;
};

const commandEndpoints: CommandEndpoint = {
    "users-list": "/users/list",
    "inbox-list": "/inbox/list"
};

function buildURL(path: string): string {
    let url = "/api";
    if (!path.match(/^\/v\d+/)) {
        url += `/${DEFAULT_VERSION}`;
    }
    url += path;
    return url;
}
