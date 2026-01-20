# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Converted from a multi-crate workspace into a single-crate workspace.
  - Migrated relevant metadata from the root `Cargo.toml` into `exn/Cargo.toml`.
  - Moved `examples/src/*.rs` into `exn/examples/`.
  - Moved `exn/*` into the root of the repository.
- Consolidated/reorganized modules:
  - Moved `ext::Ok` to `result`.
  - Renamed `ext` to `error`.
  - Renamed `impls` to `exn`.
  - Flattened `debug` and `display` into `exn`.
- Changed the `Debug` implementation of `Frame` to reduce vertical whitespace and use Unicode
  box-drawing characters (`├─` and `└─`) instead of `|->`.
  - Updated code comments featuring `Debug`-style outputs accordingly.
- Removed any indication that this crate is `no_std`-compatible.
- Replaced `derive-more` with `thiserror` in `[dev-dependencies]` in `Cargo.toml`.
  - Updated `examples` to reflect this change.
- Renamed and reorganized Rust sources under `test/`.
- Changed `rustfmt.toml` settings (e.g., "Crate" import granularity).
  - Reformatted all Rust sources with `cargo +nightly fmt`.
- Simplified `.github/ci.yml`.
- Updated `README.md`.
  - Removed non-applicable links and badges.
  - Reworded the _Overview_ section.
  - Rewrote the _Documentation_ section.
  - Credited original owners (FastLabs Developers) in the _License_ section.
- Prepended "Copyright 2026 Andrew Lehmer (github.com/80Ltrumpet)" to the copyright header of all
  modified files, where applicable.

### Removed

- Removed `.cargo/`.
- Removed `.github/semantic.yml`.
- Removed `.editorconfig`.
- Removed `licenserc.toml`.
- Removed `taplo.toml`.
- Removed `typos.toml`.
- Removed `xtask/`.

## [0.1.0] - 2026-01-19

### Added

- Forked from [fast/exn] at commit
  [85b12a4c6382611192f700b32a4556e0633becc1](https://github.com/fast/exn/commit/85b12a4c6382611192f700b32a4556e0633becc1)

[0.1.0]: https://github.com/80Ltrumpet/exn/releases/tag/v0.1.0
