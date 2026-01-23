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

use std::error::Error;

use crate::Exn;

/// Reasonable return type to use throughout an application
pub type Result<T, E> = std::result::Result<T, Exn<E>>;

/// Propagating [`Result`]s with context
pub trait ResultExt {
    /// [`Ok`] type
    type Success;

    /// Raises the [`Err`] variant as a new [`Exn`] whose context is provided by `err`.
    #[expect(clippy::missing_errors_doc, reason = "similar to `Result::map_err`")]
    fn or_raise<A, B, F>(self, err: F) -> Result<Self::Success, B>
    where
        A: Into<B>,
        B: Error + Send + Sync + 'static,
        F: FnOnce() -> A;
}

impl<T, E> ResultExt for std::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    type Success = T;

    #[track_caller]
    fn or_raise<A, B, F>(self, err: F) -> Result<Self::Success, B>
    where
        A: Into<B>,
        B: Error + Send + Sync + 'static,
        F: FnOnce() -> A,
    {
        // Note: We can't use `Result::map_err` since `#[track_caller]` on closures is currently
        // unstable.
        match self {
            Self::Ok(t) => Result::Ok(t),
            Self::Err(e) => Result::Err(Exn::new(e).raise(err().into())),
        }
    }
}

impl<T, E> ResultExt for std::result::Result<T, Exn<E>>
where
    E: Error + Send + Sync + 'static,
{
    type Success = T;

    #[track_caller]
    fn or_raise<A, B, F>(self, err: F) -> Result<Self::Success, B>
    where
        A: Into<B>,
        B: Error + Send + Sync + 'static,
        F: FnOnce() -> A,
    {
        // Note: We can't use `Result::map_err` since `#[track_caller]` on closures is currently
        // unstable.
        match self {
            Self::Ok(t) => Result::Ok(t),
            Self::Err(e) => Result::Err(e.raise(err().into())),
        }
    }
}

/// Equivalent to `Ok::<_, Exn<E>>(value)`.
///
/// This simplifies creation of an [`exn::Result`] in places where type inference cannot deduce the
/// type of the [`Err`] variant.
///
/// One might think that `exn::Result::Ok(value)` would work in such cases, but it does not.
///
/// [`exn::Result`]: Result
#[expect(clippy::missing_errors_doc, reason = "only returns `Ok`")]
#[expect(non_snake_case)]
pub fn Ok<T, E: Error + Send + Sync + 'static>(value: T) -> Result<T, E> {
    Result::Ok(value)
}
