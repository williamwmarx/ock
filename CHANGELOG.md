# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

