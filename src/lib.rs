// Copyright 2026 Andrew Lehmer (github.com/80Ltrumpet)
//
// Copyright 2025 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Context-aware [`Error`] handling
//!
//! # Examples
//!
//! ```no_run
//! use std::{
//!     error::Error,
//!     fmt::{self, Display, Formatter},
//! };
//!
//! use exn::{Result, ResultExt, bail};
//!
//! // It's recommended to define errors as structs. `Exn` will maintain the error tree
//! // automatically. Note that the `thiserror` crate can make defining errors like this more
//! // concise.
//! #[derive(Debug)]
//! struct LogicError(String);
//!
//! impl Display for LogicError {
//!     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//!         write!(f, "logic error: {}", self.0)
//!     }
//! }
//!
//! impl Error for LogicError {}
//!
//! fn do_logic() -> Result<(), LogicError> {
//!     bail!(LogicError("0 == 1".into()));
//! }
//!
//! // Errors can be `enum`s but notably don't need to chain the source error.
//! #[derive(Debug)]
//! enum AppError {
//!     Fatal { consequences: &'static str },
//!     Trivial,
//! }
//!
//! impl Display for AppError {
//!     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//!         match self {
//!             AppError::Fatal { consequences } => write!(f, "fatal error: {consequences}"),
//!             AppError::Trivial => write!(f, "trivial error"),
//!         }
//!     }
//! }
//!
//! impl Error for AppError {}
//!
//! fn main() -> Result<(), AppError> {
//!     do_logic().or_raise(|| AppError::Fatal {
//!         consequences: "math no longer works",
//!     })
//! }
//! ```
//!
//! The above program will print an error message like:
//!
//! ```text
//! fatal error: math no longer works, at src/lib.rs:63:16
//! └─ logic error: 0 == 1, at src/lib.rs:41:5
//! ```
//!
//! [`Error`]: std::error::Error

#![deny(missing_docs)]
#![warn(clippy::pedantic, clippy::map_err_ignore)]

pub mod repr;

mod error;
mod exn;
mod iter;
mod macros;
mod option;
mod result;

#[doc(inline)]
pub use self::{
    error::ErrorExt,
    exn::{Exn, Frame},
    iter::IteratorExt,
    option::OptionExt,
    repr::{ExnAny, Repr},
    result::{Ok, Result, ResultExt},
};
