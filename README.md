# ✨ flemoji

A simple tool to customize Microsoft Fluent UI emojis with given size and type written in Rust.

<details>
<summary>📖 Table of Contents</summary>

- [🚚 Installation](#-installation)
- [💡 Usage](#-usage)
  - [Help](#help)
  - [Example](#example)
- [📜 License](#-license)

</details>

## 🚚 Installation

```bash
cargo install --git https://github.com/wind-addons/flemoji
```

## 💡 Usage

### Help

```text
flemoji - Customize Microsoft Fluent UI emoji images.

Usage: flemoji.exe [OPTIONS] --width <WIDTH> --height <HEIGHT> --from <DIR> --to <DIR>

Options:
  -W, --width <WIDTH>        Sets the width of the output images
  -H, --height <HEIGHT>      Sets the height of the output images
      --from <DIR>           Sets the input directory (assets)
      --to <DIR>             Sets the output directory
  -T, --filetype <FILETYPE>  Sets the output file type (png, jpg, etc.) [default: png]
  -h, --help                 Print help
  -V, --version              Print version
```

### Example

```bash
flemoji --width 64 --height 64 --from ./assets --to ./output --type png
```

## 📜 License

MIT
