# Repository Guidelines

## Project Structure & Module Organization
- `Cargo.toml`: crate metadata and dependencies (Rust 2021).
- `src/main.rs`: entry point; orchestrates parsing, selection, and printing.
- `src/cli.rs`: CLI via `clap`; `Args` and `parse_input` (pipes, files, strings).
- `src/selector.rs`: selection ranges and regex; `Selector` parsing logic.
- `src/utils.rs`: helpers (`split`, `regex_eq`); included with `include!("utils.rs")` for reuse.
- `README.md`: install and usage examples.

## Build, Test, and Development Commands
- Build: `cargo build` (release: `cargo build --release`).
- Run example: `ps aux | cargo run -- -c pid -r 0:10`.
- Format: `cargo fmt --all` (check in CI/dev: `cargo fmt --all -- --check`).
- Lint: `cargo clippy -- -D warnings`.
- Tests: `cargo test` (unit and integration; see below).

## Coding Style & Naming Conventions
- Use rustfmt defaults (4‑space indent); run `cargo fmt` before pushing.
- Naming: functions/vars `snake_case`; types `CamelCase`; constants `SCREAMING_SNAKE_CASE`.
- Add doc comments `///` for public items and modules; keep examples small and runnable.
- Prefer iterators and borrowing to avoid unnecessary allocations; keep `main` thin and logic in modules.

## Testing Guidelines
- Framework: Rust built‑in test harness.
- Unit tests: add `#[cfg(test)] mod tests { ... }` in `src/*` (e.g., parsing in `selector.rs`, splitting in `utils.rs`).
- Integration tests: create files in `tests/` that exercise CLI behavior via `cargo run` or helper functions.

## Commit & Pull Request Guidelines
- Commits: concise, imperative subject; reference issues (e.g., `Fix selector step off‑by‑one (#42)`). Conventional Commits are welcome (`feat:`, `fix:`, `docs:`).
- PRs: describe problem, approach, and tradeoffs; link issues; include before/after example (command + sample output) when changing flags or output.
- Checks: ensure `cargo fmt`, `clippy`, `build`, and `test` pass locally before requesting review.
- Docs: update `README.md` when altering flags, defaults, or examples.

## Security & Configuration Tips
- Be mindful of user‑supplied regex and delimiters; prefer predictable patterns and avoid catastrophic backtracking.
- Handle errors with clear messages; avoid introducing new `panic!`s in non‑test code paths.

