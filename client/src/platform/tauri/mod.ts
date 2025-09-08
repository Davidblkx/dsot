import type { Platform } from "../typings.d.ts";

export const core: Platform = {
    getRuntimeName(): string {
        return "tauri";
    },
};
