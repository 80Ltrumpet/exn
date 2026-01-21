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

//! [`Error`] representations for [`Exn`] via type-erasure

mod anyhow;
mod list;
mod tree;

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
    marker::PhantomData,
};

#[doc(inline)]
pub use self::{anyhow::Anyhow, list::List, tree::Tree};
use crate::Exn;

/// [`ExnAny`] representation marker trait
///
/// This trait is used with [`ExnAny`] for type-erasing concrete [`Exn`] types into a wrapper that
/// provides [`Debug`], [`Display`], and [`Error`] implementations.
pub trait Repr {
    /// `Exn<T>` wrapper implementation
    type Impl<T>: Error + From<Exn<T>> + Send + Sync + 'static
    where
        T: Error + Send + Sync + 'static;
}

/// Type-erased [`Exn`] that implements [`Error`]
///
/// [`ExnAny`] is convertible from any [`Exn`], so it is suitable for application-level errors
/// (similar to [`anyhow::Error`]):
///
/// ```no_run
/// use std::{fmt, io};
///
/// use exn::{ErrorExt, ExnAny};
///
/// fn foo() -> exn::Result<(), fmt::Error> {
///     Err(fmt::Error.raise())
/// }
///
/// fn bar() -> exn::Result<(), io::Error> {
///     Err(io::Error::other("bar").raise())
/// }
///
/// fn main() -> Result<(), ExnAny> {
///     foo()?;
///     bar()?;
///     Ok(())
/// }
/// ```
///
/// # Representations
///
/// Any type that implements [`Repr`] may be provided as a type parameter to [`ExnAny`] ([`Tree`]
/// is the default).
///
/// [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
pub struct ExnAny<T: Repr = Tree> {
    error: Box<dyn Error + Send + Sync + 'static>,
    _repr: PhantomData<T>,
}

impl<E, T> From<Exn<E>> for ExnAny<T>
where
    E: Error + Send + Sync + 'static,
    T: Repr,
{
    fn from(exn: Exn<E>) -> Self {
        Self {
            error: Box::new(T::Impl::<E>::from(exn)),
            _repr: PhantomData,
        }
    }
}

impl<T: Repr> Debug for ExnAny<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&*self.error, f)
    }
}

impl<T: Repr> Display for ExnAny<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&*self.error, f)
    }
}

impl<T: Repr> Error for ExnAny<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}
