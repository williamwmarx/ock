# Repository Guidelines

## Project Structure & Module Organization
- `Cargo.toml`: crate metadata and dependencies (Rust 2021).
- `src/main.rs`: entry point; orchestrates parsing, selection, and printing.
- `src/cli.rs`: CLI via `clap`; `Args` and `parse_input` (pipes, files, strings).
- `src/selector.rs`: selection ranges and regex; `Selector` parsing logic.
- `src/utils.rs`: helpers (`split`, `regex_eq`); included with `include!("utils.rs")` for reuse.
- `tests/`: integration tests for CLI behavior.
- `README.md`: install and usage examples.

## Build, Test, and Development Commands
- Build: `cargo build` (release: `cargo build --release`).
- Run example: `ps aux | cargo run -- -c pid -r 0:10`.
- Format: `cargo fmt --all` (CI check: `cargo fmt --all -- --check`).
- Lint: `cargo clippy -- -D warnings`.
- Tests: `cargo test` (unit + integration).

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
- Be mindful of user‑supplied regex and delimiters; avoid catastrophic backtracking.
- Validate input and handle errors with clear messages; no new `panic!`s in production code paths.

