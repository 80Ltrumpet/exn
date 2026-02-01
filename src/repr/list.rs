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

use std::error::Error;

use crate::{Frame, Repr};

/// [`ExnAny`] representation that coerces the exception tree into an [`Error::source`] chain
///
/// The [`Debug`] representation of this [`Repr`], unlike [`Exn`], does _not_ traverse the source
/// chain.
///
/// ```
/// use std::{error::Error, io};
///
/// use exn::{ErrorExt, ExnAny, repr};
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
    type Impl<T: Error + Send + Sync + 'static> = Frame;
}
