# Changelog

All notable changes to vexr will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-05-05

### Added
- System tray icon with context menu (Set Reminder, Manage Reminders, Quit)
- Create one-time and recurring reminders (daily, weekly, monthly)
- Manage Reminders window: list, edit, and delete reminders
- Alert window: always-on-top, non-dismissible without explicit acknowledgment
- Background scheduler (30-second tick) that fires alerts for due reminders
- Local JSON storage via `tauri-plugin-store`
- GitHub Actions CI workflow (fmt, clippy, test)
- MIT license

[Unreleased]: https://github.com/your-org/vexr/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-org/vexr/releases/tag/v0.1.0
