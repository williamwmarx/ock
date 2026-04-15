# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-04-15

### Other

- Archive repo ([#65](https://github.com/williamwmarx/ock/pull/65))


## [0.1.3] - 2026-04-13

### Other

- Bump actions/upload-artifact from 4 to 7 ([#64](https://github.com/williamwmarx/ock/pull/64))
- Bump actions/checkout from 4 to 6 ([#63](https://github.com/williamwmarx/ock/pull/63))
- Bump softprops/action-gh-release from 2 to 3 ([#62](https://github.com/williamwmarx/ock/pull/62))
- Bump actions/download-artifact from 4 to 8 ([#61](https://github.com/williamwmarx/ock/pull/61))
- Bump actions/cache from 4 to 5 ([#60](https://github.com/williamwmarx/ock/pull/60))
- Bump tempfile from 3.21.0 to 3.27.0 ([#58](https://github.com/williamwmarx/ock/pull/58))
- Bump regex from 1.11.2 to 1.12.3 ([#57](https://github.com/williamwmarx/ock/pull/57))
- Bump once_cell from 1.21.3 to 1.21.4 ([#56](https://github.com/williamwmarx/ock/pull/56))
- Bump clap from 4.5.47 to 4.6.0 ([#55](https://github.com/williamwmarx/ock/pull/55))
- Add github-actions ecosystem
- Add Dependabot configuration for Cargo updates


## [0.1.2] - 2026-01-07

### Fixed

- Use is_multiple_of() to satisfy new clippy lint

### Other

- Bump lru from 0.16.0 to 0.16.3


## [0.1.1] - 2025-09-08

### Added

- Add automated release workflows and binary distribution
- Convert utils::split() to return SelectorError for consistent error handling

### Fixed

- Address code review feedback for release workflows
- Remove flaky cache assertion in thread safety test
- Update CI to run all tests together
- Apply cargo fmt to test files
- Apply cargo fmt formatting fixes
- Parse selectors as i64 and clamp invalid ranges
- Prevent division by zero panic when step=0
- Guard end-of-range modulo
- Ensure end index in range with step respects step pattern

### Other

- Update CLAUDE.md with release automation and CI/CD workflow details
- Add GitHub Actions workflow for PR testing
- Merge branch 'main' into docs/unify-claudemd-agentsmd
- Unify AGENTS.md and CLAUDE.md, update copyright year
- Update AGENTS.md and CLAUDE.md with comprehensive documentation
- Merge branch 'main' into claude/issue-19-20250905-1448
- Restore unrelated test files
- Implement Python-style negative indexing support
- Merge branch 'main' into claude/issue-15-20250905-0350
- Merge branch 'main' into claude/issue-11-20250905-0349
- Restore test files
- Change step=0 to raise clear error instead of silent conversion
- Merge pull request #8 from williamwmarx/claude/issue-7-20250905-0302
- Fix selector step value bug
- Add CLAUDE.md and AGENTS.md for AI assistant guidance
- "Claude Code Review workflow"
- "Claude PR Assistant workflow"
- Update dependencies to latest compatible versions
- Replace atty by upgrading clap
- New install command

