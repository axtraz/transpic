# transpic-core

Native Node.js bindings for fast image manipulation, powered by Rust and [napi-rs](https://napi.rs/).

## Installation

```bash
npm install @axtraz/transpic-core
```

## Usage

```javascript
const { processImage } = require("@axtraz/transpic-core");

processImage("input.jpg", {
    blur: 5,
    grayscale: true,
    huerotate: 45,
    invert: true,
    resize: "800x600",
    rotate: 90,
    output_format: "png",
});
```

The function writes the output file to the current working directory and returns the output filename.

## API

### `processImage(inputPath, options)`

| Parameter   | Type           | Description                                  |
| ----------- | -------------- | -------------------------------------------- |
| `inputPath` | `string`       | Absolute or relative path to the input image |
| `options`   | `ImageOptions` | Processing options (all optional)            |

**Returns:** `string` — the output filename.

### `ImageOptions`

| Option         | Type      | Description                                   |
| -------------- | --------- | --------------------------------------------- |
| `blur`         | `number`  | Gaussian blur sigma                           |
| `grayscale`    | `boolean` | Convert to grayscale                          |
| `huerotate`    | `number`  | Rotate hue by degrees                         |
| `invert`       | `boolean` | Invert colors                                 |
| `resize`       | `string`  | Target dimensions, e.g. `"800x600"`           |
| `rotate`       | `number`  | Rotation in degrees (e.g. `90`, `180`, `270`) |
| `outputFormat` | `string`  | Output format (see supported formats below)   |

### Supported formats

`avif`, `bmp`, `exr`, `farbfeld` (`ff`), `gif`, `hdr`, `ico`, `jpeg` (`jpg`), `png`, `pnm`, `qoi`, `tga`, `tiff` (`tif`), `webp`

If `outputFormat` is omitted, the original format is preserved.

## Supported platforms

| Platform      | Architecture |
| ------------- | ------------ |
| Windows       | x64, arm64   |
| macOS         | arm64        |
| Linux (glibc) | x64          |
| Linux (musl)  | x64          |

## License

[MIT](https://github.com/axtraz/transpic/blob/main/LICENSE)
