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

use crate::{Exn, Result};

/// Generating [`Result`]s from [`None`]
pub trait OptionExt {
    /// [`Some`] type
    type Some;

    /// Raises a new [`Exn`] if `self` is [`None`].
    #[expect(clippy::missing_errors_doc, reason = "similar to `Result::map_err`")]
    fn ok_or_raise<A, B, F>(self, err: F) -> Result<Self::Some, B>
    where
        A: Into<B>,
        B: Error + Send + Sync + 'static,
        F: FnOnce() -> A;
}

impl<T> OptionExt for Option<T> {
    type Some = T;

    #[track_caller]
    fn ok_or_raise<A, B, F>(self, err: F) -> Result<T, B>
    where
        A: Into<B>,
        B: Error + Send + Sync + 'static,
        F: FnOnce() -> A,
    {
        // Note: We can't use `Option::ok_or_else` since `#[track_caller]` on closures is currently
        // unstable.
        if let Some(t) = self {
            Ok(t)
        } else {
            Err(Exn::new(err().into()))
        }
    }
}
