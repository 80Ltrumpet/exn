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

/// Creates an [`Exn`] and returns it as an [`Err`].
///
/// # Example
///
/// ```no_run
/// use std::io::Error;
///
/// use exn::{Result, bail};
///
/// fn main() -> Result<(), Error> {
///     bail!(Error::other("bailed"));
/// }
/// ```
///
/// [`Exn`]: crate::Exn
#[macro_export]
macro_rules! bail {
    ($err:expr) => {{
        return ::core::result::Result::Err($crate::Exn::from($err));
    }};
}

/// Creates an [`Exn`] and returns it as an [`Err`] if `$cond` is false.
///
/// # Example
///
/// ```no_run
/// use std::io::{self, Error};
///
/// use exn::{Result, ensure};
///
/// fn main() -> Result<(), Error> {
///     let input = io::read_to_string(io::stdin())?;
///     ensure!(!input.is_empty(), Error::other("expected input"));
///
///     // Do stuff with `input`…
/// #   drop(input);
///
///     Ok(())
/// }
/// ```
///
/// [`Exn`]: crate::Exn
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr $(,)?) => {{
        if !bool::from($cond) {
            $crate::bail!($err)
        }
    }};
}
