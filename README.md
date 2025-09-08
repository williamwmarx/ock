# ock

[![Crates.io](https://img.shields.io/crates/v/ock.svg)](https://crates.io/crates/ock)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/williamwmarx/ock/actions/workflows/ci.yml/badge.svg)](https://github.com/williamwmarx/ock/actions/workflows/ci.yml)

`ock` is a lightweight command-line tool for slicing tabular data. It covers many everyday awk tasks with a simpler, faster interface.

The project started as an effort to learn Rust and forget awk. It became useful enough for me and I stopped maintaining it. Now, thanks to the fine folks at [Anthropic](https://www.anthropic.com/claude-code) and [OpenAI](https://openai.com/codex/) making the cost of code maintenance ~zero, the project is back under active development.

## Features
- Select columns by index or header
- Slice rows and columns with Python-like ranges
- Filter using regular expressions
- Work with custom row and column delimiters

## Installation

### From crates.io
```sh
cargo install ock
```

### Using cargo-binstall
```sh
cargo binstall ock
```

### Pre-built binaries
Download pre-built binaries for your platform from the [releases page](https://github.com/williamwmarx/ock/releases).

Available for:
- Linux (x86_64, aarch64)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

## Usage

### Select columns
```bash
ps aux | ock -c pid,command          # by header
ps aux | ock -c 1,3                  # by index
```

### Slice rows
```bash
ps aux | ock -c pid -r 0:10          # first ten rows
ps aux | ock -r -5:                  # last five rows
```

### Filter with regex
```bash
ps aux | ock -r 'python(2|3)'        # rows matching a pattern
ps aux | ock -c '/^pid|cmd$/'        # columns by regex
```

### Custom delimiters
```bash
ock -r 1:10 -c 1,5 --column-delimiter ',' data.csv
```

### Combine operations
```bash
ps aux | ock -c pid,command -r 'python'
```

Out-of-bounds selections produce a warning and no output.

## License
MIT license. See [LICENSE](LICENSE) for details.
