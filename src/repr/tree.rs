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
};

use crate::{Exn, Repr};

/// [`ExnAny`] representation that delegates directly to [`Exn`]
///
/// [`ExnAny`]: crate::ExnAny
pub struct Tree;

impl Repr for Tree {
    type Impl<T: Error + Send + Sync + 'static> = TreeExn<T>;
}

pub struct TreeExn<T: Error + Send + Sync + 'static>(Exn<T>);

impl<T: Error + Send + Sync + 'static> Debug for TreeExn<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Error + Send + Sync + 'static> Display for TreeExn<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Error + Send + Sync + 'static> Error for TreeExn<T> {}

impl<T: Error + Send + Sync + 'static> From<Exn<T>> for TreeExn<T> {
    fn from(exn: Exn<T>) -> Self {
        Self(exn)
    }
}
