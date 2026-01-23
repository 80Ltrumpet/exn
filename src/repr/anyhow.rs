// Copyright 2026 Andrew Lehmer (github.com/80Ltrumpet)
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

//! [`ExnAny`] representation for interoperation with [the `anyhow` crate]
//!
//! [`ExnAny`]: crate::ExnAny
//! [the `anyhow` crate]: https://docs.rs/anyhow/latest/anyhow/

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
    marker::PhantomData,
};

use super::{Repr, Tree};
use crate::Exn;

/// [`ExnAny`] representation for interoperation with [the `anyhow` crate]
///
/// # Example
///
/// ```no_run
/// use std::io::Error;
///
/// use anyhow::Context;
/// use exn::{ErrorExt, Exn, ExnAny, repr::Anyhow};
///
/// fn foo() -> Result<(), ExnAny<Anyhow>> {
///     let child = Error::other("child").raise();
///     let parent = child.raise(Error::other("parent"));
///     Err(parent.into())
/// }
///
/// fn main() -> anyhow::Result<()> {
///     foo().context("context")?;
///     Ok(())
/// }
/// ```
///
/// This prints something similar to the following (locations elided for clarity and brevity):
///
/// ```text
/// Error: context
///
/// Caused by:
///     parent, at …
///     └─ child, at …
/// ```
///
/// # Sub-representations
///
/// [`Anyhow`] takes an optional type parameter that controls how [`Exn`]s are represented in
///
/// [`ExnAny`]: crate::ExnAny
/// [the `anyhow` crate]: https://docs.rs/anyhow/latest/anyhow/
/// [`Exn`]: crate::Exn
pub struct Anyhow<T: Repr = Tree>(PhantomData<T>);

impl<R: Repr> Repr for Anyhow<R> {
    type Impl<T: Error + Send + Sync + 'static> = AnyhowExn<R::Impl<T>>;
}

pub struct AnyhowExn<T: Error + Send + Sync + 'static>(T);

impl<T: Error + Send + Sync + 'static> Debug for AnyhowExn<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Error + Send + Sync + 'static> Display for AnyhowExn<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // This is _not_ a typo! We want to use the underlying `Debug` representation in the
        // rendering of the source chain.
        Debug::fmt(&self.0, f)
    }
}

impl<T: Error + Send + Sync + 'static> Error for AnyhowExn<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl<T, U> From<Exn<T>> for AnyhowExn<U>
where
    T: Error + Send + Sync + 'static,
    U: Error + From<Exn<T>> + Send + Sync + 'static,
{
    fn from(exn: Exn<T>) -> Self {
        Self(exn.into())
    }
}
