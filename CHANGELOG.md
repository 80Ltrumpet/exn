# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-23

### Changed

- Relaxed the constraint on the return type of the `err` (`FnOnce`) parameter for both
  `OptionExt::ok_or_raise` and `ResultExt::or_raise` to allow anything convertible to the error
  type contained in the output `Err` variant's `Exn`.
  - This makes using these extension trait methods more ergonomic in some situations, but it may
    require more explicit type annotations in others—e.g., when used in `Iterator::map` closures.
  - ⚠️ Due to the potential need for adjusting call sites with previously unnecessary type
    annotations, this is considered a **_breaking change_**.

## [0.2.1] - 2026-01-21

### Changed

- Expanded `Exn`'s `From` implementation to accept any `Error` that is convertible to the contained
  error type (i.e., `Into<E>`).
  - This improves support for existing `Error` implementations that are (or store) `enum`s of other
    `Error`s—particularly from external crates.

## [0.2.0] - 2026-01-21

### Added

- `iter` module
  - `IteratorExt` extension trait
    - `collect_all` trait method
- `repr` module
  - `Repr` marker trait
  - `ExnAny` for type-erasure to `std::error::Error`
  - `Anyhow`, `List`, and `Tree` representations (`Repr` implementations)
- Consuming conversion methods for `Exn` and `Frame`
- Alternate `Debug` formats (via `"{:#?}"`) for `Exn` and `Frame`
- Tests for all first-party representations
- `.markdownlint.json`

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
- Swapped the order of parameters for `Exn::raise_all`.
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

[Unreleased]: https://github.com/80Ltrumpet/exn/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/80Ltrumpet/exn/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/80Ltrumpet/exn/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/80Ltrumpet/exn/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/80Ltrumpet/exn/releases/tag/v0.1.0
