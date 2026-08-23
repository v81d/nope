# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [0.3.1] - 2026-08-23

### Added

- Added a "secrets" warning when adding a regret to stop users from leaking anything sensitive into the configuration.

### Changed

- Read commands from stdin instead of passing commands as arguments in the command checker.

### Fixed

- Add serde defaults to `Config` struct so that a non-empty `config.toml` with missing fields won't permanently break the program.

## [0.3.0] - 2026-06-25

### Added

- Added enabled switch for regret scanning.

### Changed

- Update MSI installation process for Windows users.

## [0.2.1] - 2026-04-03

### Fixed

- Fixed multi-word command support for PowerShell.

## [0.2.0] - 2026-04-03

### Added

- Added (experimental) support for PowerShell.

### Changed

- Updated docmuentation.
