# Contributing to vexr

Thank you for your interest in contributing!

## Getting Started

1. Fork the repository and clone your fork.
2. Install prerequisites (see README.md).
3. Create a feature branch: `git checkout -b feat/your-feature`
4. Make your changes and verify the build: `cargo tauri build --no-bundle`
5. Run the lints: `cargo fmt --check && cargo clippy -- -D warnings`
6. Commit using [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` new feature
   - `fix:` bug fix
   - `docs:` documentation only
   - `chore:` build/tooling changes
7. Open a pull request against `main`.

## Code Style

- Rust code is formatted with `rustfmt` (default settings).
- All clippy warnings are treated as errors in CI.
- Keep frontend HTML/JS/CSS in `src/`; keep Rust logic in `src-tauri/src/`.

## Reporting Bugs

Use the **Bug Report** issue template. Include:
- OS and version
- Steps to reproduce
- Expected vs actual behaviour
- Any relevant log output

## Suggesting Features

Use the **Feature Request** issue template. Describe the use case before proposing a solution.

## Security Issues

Please **do not** open a public issue for security vulnerabilities. See [SECURITY.md](SECURITY.md).
