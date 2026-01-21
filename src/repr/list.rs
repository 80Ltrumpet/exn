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

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
    marker::PhantomData,
    panic::Location,
};

use crate::{Exn, Frame, Repr};

/// [`ExnAny`] representation that coerces the exception tree into an [`Error::source`] chain
/// by recursively assigning only the first child frame as the source of its parent frame
///
/// The [`Debug`] representation of this [`Repr`], unlike [`Exn`], does _not_ traverse the source
/// chain.
///
/// ```
/// use std::{error::Error, io};
///
/// use exn::{ExnAny, repr};
///
/// fn make_exn() -> exn::Result<(), io::Error> {
///     let child = io::Error::other("child").raise();
///     Err(child.raise(io::Error::other("parent")))
/// }
///
/// fn make_exn_list() -> Result<(), ExnAny<repr::List>> {
///     Ok(make_exn()?)
/// }
///
/// // Note that `repr::Tree` is the default, so the type parameter is technically redundant.
/// fn make_exn_tree() -> Result<(), ExnAny<repr::Tree>> {
///     Ok(make_exn()?)
/// }
///
/// fn main() {
///     let as_list = make_exn_list().unwrap_err();
///     println!("{:?}", as_list);
///     // The source chain is stll accessible.
///     println!("{:?}", as_list.source().unwrap());
///     println!();
///     println!("{:?}", make_exn_tree().unwrap_err());
/// }
/// ```
///
/// This produces output similar to the following (locations elided for clarity and brevity):
///
/// ```text
/// parent, at …
/// child, at …
///
/// parent, at …
/// └─ child, at …
/// ```
///
/// [`ExnAny`]: crate::ExnAny
pub struct List;

impl Repr for List {
    type Impl<T> = ListExn<T>
    where
        T: Error + Send + Sync + 'static;
}

pub struct ListExn<T>
where
    T: Error + Send + Sync + 'static,
{
    frame: Box<ListFrame>,
    _t: PhantomData<T>,
}

impl<T> Debug for ListExn<T>
where
    T: Error + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            f.debug_struct("ListExn")
                .field("frame", &*self.frame)
                .finish_non_exhaustive()
        } else {
            Debug::fmt(&self.frame, f)
        }
    }
}

impl<T> Display for ListExn<T>
where
    T: Error + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.frame, f)
    }
}

impl<T> Error for ListExn<T> where T: Error + Send + Sync + 'static {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.frame.source()
    }
}

impl<T> From<Exn<T>> for ListExn<T>
where
    T: Error + Send + Sync + 'static,
{
    fn from(exn: Exn<T>) -> Self {
        Self {
            frame: Box::new(exn.into_frame().into()),
            _t: PhantomData,
        }
    }
}

struct ListFrame {
    error: Box<dyn Error + Send + Sync + 'static>,
    location: &'static Location<'static>,
    source: Option<Box<ListFrame>>,
}

impl ListFrame {
    fn as_dyn_error(&self) -> &(dyn Error + 'static) {
        self
    }
}

impl Debug for ListFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            f.debug_struct("ListFrame")
                .field("error", &*self.error)
                .field("location", self.location)
                .field("source", &self.source)
                .finish()
        } else {
            write!(
                f,
                "{}, at {}:{}:{}",
                &*self.error,
                self.location.file(),
                self.location.line(),
                self.location.column()
            )
        }
    }
}

impl Display for ListFrame {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&*self.error, f)
    }
}

impl Error for ListFrame {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(Self::as_dyn_error)
    }
}

impl From<Frame> for ListFrame {
    fn from(frame: Frame) -> Self {
        let location = frame.location();
        let (error, children) = frame.consume();
        let source = children
            .into_iter()
            .next()
            .map(|frame| Box::new(frame.into()));
        Self {
            error,
            location,
            source,
        }
    }
}
