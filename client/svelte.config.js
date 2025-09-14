// deno-lint-ignore-file no-process-global
// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import * as dotenv from "dotenv";

dotenv.config({ path: "./.env" });

// @ts-ignore Deno doesn't have process global
const isWeb = process.env.PLATFORM_MODE !== "tauri";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({
            fallback: "index.html",
        }),
        alias: {
            $platform: isWeb
                ? "src/platform/web/mod.ts"
                : "src/platform/tauri/mod.ts",
        },
    },
};

export default config;
