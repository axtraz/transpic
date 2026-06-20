# transpic

> Fast image manipulation — Rust core, Node.js bindings, CLI.

## Installation

### CLI

```bash
npm install -g @axtraz/transpic-cli
```

### Node.js (core)

```bash
npm install @axtraz/transpic-core
```

---

## Features

- **Blur** — Gaussian blur with configurable intensity
- **Grayscale** — Convert to grayscale
- **Invert** — Invert image colors
- **Resize** — Resize to any `WxH` dimension
- **Rotate** — Rotate by 90, 180, or 270 degrees
- **Format conversion** — Output to any supported format

Supported formats: `avif`, `bmp`, `exr`, `farbfeld`, `gif`, `hdr`, `ico`, `jpeg`/`jpg`, `png`, `pnm`, `qoi`, `tga`, `tiff`/`tif`, `webp`

---

## CLI usage

```bash
transpic --path <image> [options]
```

| Flag | Description | Example |
|---|---|---|
| `--path` | Path to the input image **(required)** | `--path photo.jpg` |
| `--format` | Output format | `--format webp` |
| `--blur <f32>` | Blur intensity | `--blur 2.5` |
| `--grayscale` | Convert to grayscale | `--grayscale` |
| `--invert` | Invert colors | `--invert` |
| `--resize <WxH>` | Resize dimensions | `--resize 1280x720` |
| `--rotate <deg>` | Rotati‡on angle (90, 180, 270) | `--rotate 90` |

At least one action flag is required.

```bas‡h
# Convert to WebP
transpic --path photo.jpg --format webp

# Resize and grayscale
transpic --path photo.png --resize 800x600 --grayscale

# Blur, invert, and save as AVIF
transpic --path photo.jpeg --blur 1.5 --invert --format avif
```

---

## Node.js API

```typescript
import { processImage } from 'transpic-core'

const output = processImage('photo.jpg', {
  grayscale: false,
  invert: false,
  blur: 2.0,
  resize: '800x600',
  rotate: 90,          // 90 | 180 | 270
  outputFormat: 'webp' // defaults to source format
})

console.log(`Saved as: ${output}`)
```

---

## License

Transpic is free and open-source and is licensed under [MIT License](https://github.com/axtraz/transpic/blob/main/LICENSE).
