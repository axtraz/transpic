# transpic-cli

A fast image manipulation CLI tool.

## Installation

```bash
npm install -g transpic-cli
```

## Usage

```bash
transpic --path <image> [options]
```

At least one action flag is required.

## Options

| Flag | Type | Description |
|------|------|-------------|
| `--path`, `-p` | `string` | Path to the input image *(required)* |
| `--format` | `string` | Convert output format (`png`, `jpg`, `webp`, …) |
| `--blur` | `float` | Apply blur with given intensity |
| `--grayscale` | `boolean` | Convert image to grayscale |
| `--invert` | `boolean` | Invert image colors |
| `--resize` | `WxH` | Resize image (e.g. `800x600`) |
| `--rotate` | `90\|180\|270` | Rotate image by given degrees |

## Examples

```bash
# Convert to WebP
transpic --path image.png --format webp

# Resize and convert
transpic --path photo.jpg --resize 1280x720 --format webp

# Grayscale + blur
transpic --path image.png --grayscale --blur 2

# Rotate and invert
transpic --path image.jpg --rotate 90 --invert
```

## License

[MIT](https://github.com/axtraz/transpic/blob/main/LICENSE)