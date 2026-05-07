# Copilot Agents for vexr

This document defines specialized Copilot agent modes for the vexr project. Agents are configured to provide domain-focused assistance and enforce project conventions.

## Project Overview
**vexr** is a cross-platform reminder application built with Rust + Tauri v2, featuring a vanilla JS frontend. The project follows OSS best practices including MIT licensing, clean architecture, and comprehensive CI/CD workflows.

---

## Agent: `rust-backend`

**Purpose**: Tauri backend development (Rust commands, scheduler, data layer)

**Activation**: Files matching `src-tauri/src/**/*.rs`, `src-tauri/Cargo.toml`

**Expertise**:
- Rust idioms and error handling
- Tauri command patterns and IPC
- Data persistence with `tauri-plugin-store`
- Async/Tokio task spawning
- UUID generation and DateTime handling
- Cross-platform considerations

**Guidelines**:
- Prioritize clarity and type safety
- Document public APIs with doc comments (`///`)
- Use the `reminders.rs` module for data layer logic
- Ensure all Tauri commands in `commands.rs` are properly typed
- Follow Rust 2021 edition conventions

**Tool Restrictions**: None

---

## Agent: `frontend-ui`

**Purpose**: Frontend development (HTML, vanilla JS, CSS)

**Activation**: Files matching `src/*.html`, inline `<script>` and `<style>` blocks

**Expertise**:
- Vanilla JavaScript (ES6+)
- HTML semantic markup
- CSS layout and responsive design
- Tauri JS API invocation (`invoke()`)
- Browser APIs (Intl for localization, Date handling)
- Accessible UI patterns

**Guidelines**:
- Use semantic HTML5 elements
- Keep CSS modular and avoid global state
- Handle all `invoke()` calls with proper error handling
- Format dates/times in local timezone using Intl API
- Test in desktop context (not browser DevTools assumptions)

**Tool Restrictions**: None

---

## Agent: `tauri-config`

**Purpose**: Tauri configuration, build, and CI/CD

**Activation**: Files matching `src-tauri/tauri.conf.json`, `.github/workflows/**/*.yml`, `src-tauri/build.rs`, `src-tauri/Cargo.toml`

**Expertise**:
- `tauri.conf.json` schema and window/tray configuration
- GitHub Actions workflows
- Build optimization and release channels
- Platform-specific settings (Linux, macOS, Windows)
- Cargo build configuration

**Guidelines**:
- Maintain security-conscious capability definitions
- Ensure CI workflows are reproducible and fast
- Document all non-obvious configuration choices
- Test platform-specific behavior

**Tool Restrictions**: None

---

## Agent: `project-lead`

**Purpose**: Project-wide coordination, architecture decisions, OSS practices

**Activation**: Files matching `README.md`, `CONTRIBUTING.md`, `docs/**/*.md`, `CHANGELOG.md`, `.gitignore`, `LICENSE`

**Expertise**:
- Project roadmap and phase planning
- OSS best practices (contributor guidelines, licensing, code of conduct)
- Documentation standards
- Semantic versioning
- Release management

**Guidelines**:
- Maintain clarity for external contributors
- Document architectural decisions
- Keep README up-to-date with build/run instructions
- Follow conventional commit patterns for clarity

**Tool Restrictions**: None

---

## Cross-Agent Conventions

### Code Quality
- All code must pass `cargo fmt` and `cargo clippy`
- New public APIs require doc comments
- Unsafe code must be justified with comments
- Tests accompany non-trivial functionality

### Commits & Changes
- Use conventional commit format: `type(scope): subject`
- Example: `feat(scheduler): add daily recurrence logic`
- Keep commits atomic and reviewable
- Update CHANGELOG.md for user-facing changes

### Documentation
- README.md: project overview, quick-start, build instructions
- CONTRIBUTING.md: dev setup, PR process, code standards
- Inline comments: clarify non-obvious logic only
- Doc comments (`///`): all public APIs

### Testing
- Unit tests for business logic (reminder recurrence, CRUD)
- Integration tests for Tauri commands
- CI enforces `cargo test` on all PRs

---

## Invoking Agents

Use the `@agent` mention in Copilot Chat:
- `@rust-backend` for Rust/Tauri backend work
- `@frontend-ui` for HTML/JS/CSS
- `@tauri-config` for configuration and CI
- `@project-lead` for architecture and OSS practices

Or specify directly in `.instructions.md` via frontmatter.

