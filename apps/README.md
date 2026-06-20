# transpic

Desktop GUI for [transpic](https://github.com/axtraz/transpic) — a fast, native image processing pipeline built on Rust. Chain operations, preview the result live, and export to any supported format.

Built with [Tauri](https://tauri.app/) + Rust, powered by `transpic-core`.

## Features

- **Pipeline-based editing** — stack operations (blur, brighten, grayscale, hue rotate, invert, resize, rotate) and toggle them on/off individually
- **Live preview** — see the result before exporting
- **Multi-format export** — PNG, JPEG, WebP, AVIF, GIF, BMP, TIFF, ICO, TGA, QOI, PNM, HDR, EXR, Farbfeld
- **CLI command preview** — every pipeline maps to an equivalent `transpic-cli` command you can copy and reuse in scripts
- **Native performance** — image processing runs in Rust, not JS

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Node.js](https://nodejs.org/) 22+
- [pnpm](https://pnpm.io/) 11+
- Platform-specific Tauri dependencies — see the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

## Development

```bash
pnpm install
pnpm tauri dev
```

## Building

```bash
pnpm tauri build
```

## Project structure

```
apps/
├── src/            # Frontend (UI, pipeline state, preview)
├── src-tauri/      # Rust backend — Tauri commands, image processing glue
└── README.md
```

The actual image processing logic lives in `transpic-core`; this app is a thin Tauri shell around it.

## Related packages

- `transpic-core` — Rust image processing library + Node.js (napi) bindings
- `transpic-cli` — command-line interface for the same pipeline

## License

[GPL-3.0](https://github.com/axtraz/transpic/blob/main/apps/LICENSE)