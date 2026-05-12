# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - yyyy-mm-dd

## [1.1.0] - 2026-05-12

### Added

- [ci] GitHub CI to test and build `wassel-cli`
- [ci] GitHub CI to make a new release
- [ci] Add cargo-binstall support for `wassel-cli`

### Fixed

- [ci] Make check and release workflows separate

[1.1.0]: https://github.com/wasselteam/wassel/releases/tag/v1.1.0

## [1.0.0] - 2026-05-12

### Added

- [runtime] Config variables support for plugins
- [interface] PostgreSQL interface for plugins via WASM component model
- [admin] Admin dashboard with resource consumption monitoring and plugin list
- [admin] Admin dashboard migrated to Svelte
- [admin] Admin dashboard layout built with Svelte
- [admin] Log aggregation for runtime events
- [admin] Server-Sent Events (SSE) support
- [runtime] `build.data` support for plugin build pipeline
- [stack] Parallel plugin stack building for improved performance
- [stack] `wassel-cli` with `stack` subcommand for managing plugin stacks
- [stack] Plugin building and serving functionality
- [interface] HTTP client support for outgoing requests from plugins
- [runtime] Streaming HTTP body support
- [runtime] Plugin configuration and base URL support
- [runtime] Server configuration via `config.toml`
- [runtime] Tracing-based logging
- [runtime] Plugin instance-per-connection isolation (new instance for every connection)
- [interface] Passing HTTP requests to plugins via `wasi:http@0.2.3`

### Changed

- [interface] Relocated WIT directory to repository root; added config to WIT deps
- [runtime] Switched to wasmtime release from crates.io instead of git dependency

### Fixed

- [runtime] Requests to `/` incorrectly returning 404
- [interface] Python plugin compatibility by removing unsupported `cli` and `clocks` imports

[1.0.0]: https://github.com/wasselteam/wassel/releases/tag/v1.0.0
