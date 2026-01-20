# Context-aware Rust `Error` handling

[![Apache 2.0 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[license-badge]: https://img.shields.io/crates/l/exn
[license-url]: LICENSE
[actions-badge]: https://github.com/80Ltrumpet/exn/workflows/CI/badge.svg
[actions-url]:https://github.com/80Ltrumpet/exn/actions?query=workflow%3ACI

## Overview

`exn` provides the missing context APIs for Rust's `std::error::Error`.

It organizes errors as a tree structure, allowing you to easily access the root cause and all
related errors with their context.

## Documentation

Build and peruse the docs via `cargo doc`.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE). The copyright for the
original source is held by [FastLabs Developers](https://github.com/fast/exn).
