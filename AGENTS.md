# AGENTS.md - Repository Guidelines for AI Assistants

This file provides specific guidance for AI assistants (like Claude) working on the `ock` project.

## Project Structure & Module Organization
- `Cargo.toml`: Crate metadata and dependencies (Rust 2021 edition)
  - Main dependencies: clap 4.5.4, regex 1.7.0, once_cell 1.21.3, lru 0.16.0
  - Dev dependencies: tempfile 3.8.0
- `src/main.rs`: Entry point containing main parsing logic and output formatting
  - Handles input source detection (stdin, file, or literal text)
  - Implements column alignment for pretty-printed output
  - Contains `SelectionState` struct for tracking selection iteration
- `src/cli.rs`: CLI argument parsing using clap with derive features
  - Defines `Args` struct for command-line interface
  - Implements `parse_input` function for handling various input sources
- `src/selector.rs`: Selector struct and parsing logic for row/column selection syntax
  - Supports Python-like slicing syntax and regex patterns
  - Contains selector matching logic and error handling
- `src/utils.rs`: Utility functions included via `include!()` macro
  - Provides `regex_compare` and `split` helper functions
  - Functions for text processing and regex matching
- Test files: `*_tests.rs` files for unit tests per module
- `tests/integration_test.rs`: End-to-end CLI behavior testing
- `README.md`: Installation instructions and usage examples
- `CLAUDE.md`: Comprehensive project documentation for Claude Code
- `AGENTS.md`: This file - guidelines for AI assistants

## Build, Test, and Development Commands
- **Build**: 
  - Debug: `cargo build`
  - Release: `cargo build --release` (applies aggressive optimizations from Cargo.toml)
- **Run Examples**: 
  - `ps aux | cargo run -- -c pid -r 0:10`
  - `ps aux | cargo run -- -c 2,11`
  - `cargo run -- -c 1,3,5 --column-delimiter "," data.csv`
- **Format**: 
  - `cargo fmt` or `cargo fmt --all`
  - Check only: `cargo fmt --check`
- **Lint**: `cargo clippy` (enforce warnings as errors with `-- -D warnings`)
- **Tests**: 
  - All tests: `cargo test`
  - Unit tests only: `cargo test --lib`
  - Integration tests: `cargo test --test integration_test`
  - Specific test: `cargo test test_name`
- **Installation**: `cargo install --path .` (for local testing)

## Coding Style & Naming Conventions
- Use rustfmt defaults (4‑space indent); run before pushing.
- Names: functions/vars `snake_case`, types `CamelCase`, consts `SCREAMING_SNAKE_CASE`.
- Add `///` docs for public items; keep examples small and runnable.
- Prefer iterators and borrowing; keep `main` thin and move logic into modules.

## Testing Guidelines
- Framework: Rust built‑in test harness.
- Unit tests in `src/*` (`#[cfg(test)] mod tests`) covering parsing, splitting, selection.
- Integration tests in `tests/` exercising CLI behavior end‑to‑end.
- Run locally with `cargo test`; avoid introducing new `panic!` in non‑test paths.

## Commit & Pull Request Guidelines
- Commits: concise, imperative subject; reference issues (e.g., `Fix selector step off‑by‑one (#42)`).
- Conventional Commits welcome (`feat:`, `fix:`, `docs:`).
- PRs: describe problem, approach, and tradeoffs; link issues; include before/after examples when changing flags or output.
- Ensure `cargo fmt`, `clippy`, build, and tests pass before review.
- Update `README.md` when altering flags, defaults, or examples.

## Security & Configuration Tips
- Be mindful of user-supplied regex and delimiters; avoid catastrophic backtracking
- Validate input and handle errors with clear messages; no new `panic!`s in production code paths
- The project uses `once_cell` and `lru` for regex caching to improve performance
- Test regex patterns thoroughly, especially with edge cases and malformed input

## Key Implementation Notes for AI Assistants
- The selector system supports both numeric indices (1-based) and regex patterns
- Python-like slicing syntax is supported (e.g., `1:10:2` for range with step)
- Default delimiters: `\n` for rows, `\s` for columns (both are regex patterns)
- The `utils.rs` file is included via `include!()` macro in other modules
- Out-of-bounds indices are silently ignored (documented behavior)
- Empty input returns empty output without errors
- Step value of 0 in ranges is treated as step 1
- The project has known issues documented in CLAUDE.md that should be considered

## Performance Considerations
- Release builds use aggressive optimizations (size optimization, LTO, single codegen unit)
- Regex patterns are cached using LRU cache for performance
- The binary strips symbols and uses `panic=abort` for smaller size
- Consider memory usage when working with large datasets

