import { defineConfig } from "oxlint";

// https://oxc.rs/docs/guide/usage/linter/config.html
export default defineConfig({
    options: { typeAware: true, typeCheck: true },
    plugins: ["eslint", "typescript", "unicorn", "oxc", "jsdoc", "node", "promise"],
    env: {
        builtin: true,
        node: true,
    },
    categories: {
        correctness: "error",
        nursery: "warn",
        pedantic: "off",
        perf: "warn",
        restriction: "off",
        style: "off",
        suspicious: "warn",
    },
    rules: {
        "no-useless-concat": "off",
        "no-shadow": "off",
        "unicorn/no-empty-file": "off",
    },
    ignorePatterns: ["target", "node_modules", "index.js", "package-template.wasi-browser.js", "package-template.wasi.cjs", "wasi-worker-browser.mjs", "wasi-worker.mjs", ".pnpm-lock.yaml"],
});