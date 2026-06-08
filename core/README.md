# `transpic-core`

High-performance image manipulation library powered by Rust — the core engine behind [`transpic-cli`](https://www.npmjs.com/package/transpic-cli).

## Install

```bash
npm install transpic-core
# or
pnpm add transpic-core
```

## Usage

```ts
import { processImage } from "transpic-core";

const output = processImage("path-to-my-image", {
    blur: 4.5,
    grayscale: true,
    invert: false,
    resize: undefined,
    rotate: undefined,
    outputFormat: "png",
});
```

> Full API reference available in the documentation (Coming soon).

## Supported platforms

| Platform | Architecture                   | Support |
| -------- | ------------------------------ | ------- |
| Windows  | x64, x86, arm64                | ✅      |
| macOS    | x64, arm64                     | ✅      |
| Linux    | x64, arm64, armv7 (gnu + musl) | ✅      |
| Android  | arm64, armv7                   | ✅      |
| FreeBSD  | x64                            | ✅      |
| WASI     | wasm32                         | ✅      |

## License

[MIT](https://github.com/axtraz/transpic/blob/main/LICENSE)
