# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`ock` is a command-line utility for working with table-like data, serving as a simpler and faster replacement for most awk use cases. It's written in Rust and uses a selector-based approach to extract specific rows and columns from structured text data.

## Core Architecture

### Module Structure
- `main.rs` - Entry point containing the main parsing logic and output formatting
- `cli.rs` - Command-line argument parsing using the `clap` crate
- `selector.rs` - Selector struct and parsing logic for row/column selection syntax
- `utils.rs` - Utility functions for regex comparison and text splitting (included via `include!()` in other modules)

### Key Components

#### Selector System (`selector.rs`)
- Implements Python-like slicing syntax (e.g., `1:10:2`)
- Supports both numeric indices and regex patterns for selection
- Each selector has start/end indices or regex patterns, plus a step value
- Selectors are parsed from comma-separated strings into vectors

#### Input Processing Flow (`main.rs`)
1. Parse CLI arguments and read input (stdin, file, or direct text)
2. Parse row and column selectors into `Selector` structs
3. Process first row to determine column indices to extract
4. Iterate through rows, applying selectors to filter data
5. Format output with aligned columns for pretty printing

## Development Commands

### Build
```bash
cargo build           # Debug build
cargo build --release # Optimized release build
```

### Test
```bash
cargo test           # Run tests (currently no tests implemented)
```

### Format & Lint
```bash
cargo fmt            # Format code
cargo fmt --check    # Check formatting without changes
cargo clippy         # Run linter
```

### Run Examples
```bash
# Select column 2 from process list
ps aux | cargo run -- -c 2

# Select rows 1-10 and columns 1,5
cargo run -- -r 1:10 -c 1,5 data.txt

# Use regex to select rows containing "python"
ps aux | cargo run -- -r "python"
```

## Key Implementation Details

### Delimiter Handling
- Default row delimiter: `\n` (newline)
- Default column delimiter: `\s` (whitespace)
- Custom delimiters supported via `--row-delimiter` and `--column-delimiter`
- Delimiters are treated as regex patterns

### Selector Syntax
- Single value: `5` (selects item at index 5)
- Range: `1:10` (selects items 1 through 10)
- Range with step: `1:10:2` (selects every 2nd item from 1 to 10)
- Regex: `pid` (case-insensitive partial match)
- Multiple selectors: `1,5,10` or `name,pid`

### Input Sources
The CLI intelligently determines input source:
1. If no input provided: reads from stdin (for piped input)
2. If input is an existing file path: reads file contents
3. Otherwise: treats input as literal text to parse