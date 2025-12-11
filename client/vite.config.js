import { defineConfig, createLogger } from "vite";
import vue from "@vitejs/plugin-vue";
import vuetsx from "@vitejs/plugin-vue-jsx";
import process from "node:process";
import path from "node:path";

const host = process.env.TAURI_DEV_HOST;

const platform = process.env.PLATFORM_MODE || "web";
console.log(`Running on platform: ${platform}`);

// https://github.com/denoland/deno/issues/28850#issuecomment-2944165430
const logger = createLogger();
const logError = logger.error;

// deno-lint-ignore ban-ts-comment
// @ts-ignore
logger.error = (msg, options) => {
    if (msg.includes("http proxy error:") && msg.includes("ext:deno_fetch/23_request.js:619:7")) return;
    logError(msg, options);
};

export default defineConfig(() => {
    // https://vite.dev/config/
    return {
        customLogger: logger,
        plugins: [vue(), vuetsx()],
        // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
        //
        // 1. prevent Vite from obscuring rust errors
        clearScreen: false,
        // 2. tauri expects a fixed port, fail if that port is not available
        server: {
            port: 1420,
            strictPort: true,
            host: host || false,
            hmr: host
                ? {
                    protocol: "ws",
                    host,
                    port: 1421,
                }
                : undefined,
            watch: {
                // 3. tell Vite to ignore watching `src-tauri`
                ignored: ["**/src-tauri/**"],
            },
            proxy: {
                "/api": {
                    changeOrigin: true,
                    target: "http://localhost:6677/api",
                    rewrite: (path) => path.replace(/^\/api/, ""),

                },
            },
            allowedHosts: true
        },
        resolve: {
            alias: [
                {
                    find: "$platform",
                    replacement: path.resolve(import.meta.dirname, `src/platform/${platform}/mod.ts`),
                },
                {
                    find: /^\$css\/(.*).css$/,
                    replacement: path.resolve(import.meta.dirname, "css/$1.css"),
                },
                {
                    find: /^\$infra\/(.*).ts$/,
                    replacement: path.resolve(import.meta.dirname, `src/infra/$1.ts`),
                }
            ]
        }
    };
});
