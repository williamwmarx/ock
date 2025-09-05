# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`ock` is a command-line utility for working with table-like data, serving as a simpler and faster replacement for most awk use cases. It's written in Rust and uses a selector-based approach to extract specific rows and columns from structured text data.

## Development Commands

### Build
```bash
cargo build           # Debug build
cargo build --release # Optimized release build (applies aggressive optimizations from Cargo.toml)
```

### Test
```bash
cargo test                           # Run all tests (unit and integration)
cargo test --test integration_test   # Run integration tests only
cargo test test_name                 # Run specific test by name
cargo test --lib                    # Run unit tests only
```

### Format & Lint
```bash
cargo fmt            # Format code
cargo fmt --check    # Check formatting without changes
cargo clippy         # Run linter
```

### Installation
```bash
cargo install --path .  # Install ock locally for testing
```

## Core Architecture

### Module Structure
- `main.rs` - Entry point containing the main parsing logic and output formatting
  - Handles input source detection (stdin, file, or literal text)
  - Manages the row/column selection pipeline
  - Implements column alignment for pretty-printed output
- `cli.rs` - Command-line argument parsing using the `clap` crate
  - Defines CLI interface with Args struct
  - Implements input parsing logic (parse_input function)
- `selector.rs` - Selector struct and parsing logic for row/column selection syntax
  - Implements Python-like slicing syntax (e.g., `1:10:2`)
  - Supports both numeric indices and regex patterns
  - Contains selector matching logic for rows and columns
- `utils.rs` - Utility functions for regex comparison and text splitting
  - Included via `include!()` macro in other modules
  - Provides `regex_compare` and `split` helper functions

### Test Organization
- Unit tests are embedded in each module using `#[cfg(test)]` modules
  - `cli_tests.rs` - Tests for CLI parsing
  - `main_tests.rs` - Tests for main logic functions
  - `selector_tests.rs` - Tests for selector parsing and matching
  - `utils_tests.rs` - Tests for utility functions
- Integration tests in `tests/integration_test.rs`
  - End-to-end tests simulating actual CLI usage
  - Tests for various data processing scenarios

## Key Implementation Details

### Input Processing Flow
1. Parse CLI arguments via `clap`
2. Determine input source (stdin detection, file check, or literal text)
3. Parse row and column selectors into `Selector` structs
4. Split input into rows using row delimiter regex
5. For each row:
   - Check if it matches row selectors
   - Split into columns and extract matching column indices
   - Collect selected cells
6. Format output with aligned columns for pretty printing

### Selector System
- Single value: `5` (selects item at index 5, 1-based)
- Range: `1:10` (selects items 1 through 10, inclusive)
- Range with step: `1:10:2` (selects every 2nd item from 1 to 10)
- Regex: `pid` (case-insensitive partial match against content)
- Multiple selectors: `1,5,10` or `name,pid` (comma-separated)
- Mixed numeric and regex: Supported in the same selector list

### Delimiter Handling
- Default row delimiter: `\n` (newline)
- Default column delimiter: `\s` (whitespace regex)
- Custom delimiters supported via `--row-delimiter` and `--column-delimiter`
- Delimiters are treated as regex patterns
- Special handling for regex metacharacters in delimiters

### Edge Cases and Behavior
- Empty input: Returns empty output
- Out-of-bounds indices: Silently ignored (no error)
- Regex selectors that don't match: No output for those selectors
- Invalid ranges (start > end): Returns empty selection
- Step value of 0: Treated as step 1
- Whitespace-only lines with default delimiter: Filtered out by split()

## Known Issues and TODOs
- Step values in selectors have a documented bug (see test_row_range_with_step comment)
- Out-of-bounds column indices return entire row instead of empty (see test_out_of_bounds_indices)
- Consider error handling for invalid selector syntax instead of silent failures

## Performance Optimizations
- Release profile uses aggressive optimizations:
  - Strip symbols for smaller binary
  - Optimize for size (`opt-level = "z"`)
  - Link-time optimization enabled
  - Single codegen unit for better optimization
  - Panic=abort for smaller binary

## Common Usage Patterns
```bash
# Select specific columns from process list
ps aux | ock -c 2,11

# Filter rows by regex and select columns
ps aux | ock -r python -c pid,command

# Process CSV files
ock -c 1,3,5 --column-delimiter "," data.csv

# Select row ranges with step
ock -r 1:100:10 large_file.txt  # Every 10th row from 1-100
```