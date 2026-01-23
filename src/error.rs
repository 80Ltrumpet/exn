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

/// Raising [`Error`]s as [`Exn`]s
pub trait ErrorExt: Error + Send + Sync + 'static {
    /// Raises this [`Error`] as a new [`Exn`], capturing the location of the callsite.
    #[track_caller]
    fn raise(self) -> Exn<Self>
    where
        Self: Sized,
    {
        Exn::new(self)
    }
}

impl<T: Error + Send + Sync + 'static> ErrorExt for T {}
