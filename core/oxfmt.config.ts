import { defineConfig } from "oxfmt";

// https://oxc.rs/docs/guide/usage/formatter/config.html
export default defineConfig({
    printWidth: 120,
    arrowParens: "avoid",
    tabWidth: 4,
    sortTailwindcss: true,
    sortImports: {
        newlinesBetween: false,
    },
    ignorePatterns: [
        "target",
        "node_modules",
        "index.js",
        "transpic.wasi-browser.js",
        "transpic.wasi.cjs",
        "browser.js",
        "package-template.wasi-browser.js",
        "package-template.wasi.cjs",
        "wasi-worker-browser.mjs",
        "wasi-worker.mjs",
        ".pnpm-lock.yaml",
    ],
});