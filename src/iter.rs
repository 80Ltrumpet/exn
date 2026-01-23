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

//! [`Iterator`] extension trait

/// Extension trait for [`Iterator`]s of [`Result`]s
pub trait IteratorExt<T, E>: Iterator<Item = Result<T, E>> {
    /// Transforms this [`Iterator`] of [`Result`]s into a [`Result`] of _collections_.
    ///
    /// This method produces a [`Result`] where _both_ the [`Ok`] and [`Err`] variants are
    /// [`FromIterator`].
    ///
    /// Unlike [`Result`]'s [`FromIterator`] implementation, this method is _not_ short-circuiting;
    /// it will always consume all items in `self`.
    ///
    /// # What does this have to do with [`Exn`]?
    ///
    /// This method pairs well with [`Exn::raise_all`]:
    ///
    /// ```no_run
    /// use std::io::Error;
    ///
    /// use exn::{Exn, IteratorExt, Result, ResultExt};
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Suppose we need to open several files. It is more helpful for debugging to know all
    ///     // of the failures rather than only the first one.
    ///     let files: Vec<_> = ["a/b", "c/d", "e/f", "g/h", "i/j"]
    ///         .into_iter()
    ///         .map(|path| {
    ///             std::fs::File::open(path)
    ///                 .or_raise(|| Error::other(format!("failed to open {path}")))
    ///         })
    ///         .collect_all::<_, Vec<Exn<Error>>>()
    ///         .map_err(|children| Exn::raise_all(children, Error::other("example")))?;
    ///
    ///     // Do stuff with files…
    /// #   drop(files);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This yields an [`Exn`] whose [`Debug`] representation looks like this (locations elided for
    /// clarity and brevity):
    ///
    /// ```text
    /// example, at …
    /// ├─ failed to open a/b, at …
    /// │  └─ No such file or directory (os error 2), at …
    /// ├─ failed to open c/d, at …
    /// │  └─ No such file or directory (os error 2), at …
    /// (etc.)
    /// ```
    ///
    /// # Errors
    ///
    /// Similar to [`Result`]'s [`FromIterator`] implementation, if any item is [`Err`], this
    /// method will return [`Err`].
    ///
    /// [`Exn`]: crate::Exn
    /// [`Exn::raise_all`]: crate::Exn::raise_all
    /// [`Debug`]: std::fmt::Debug
    fn collect_all<A, B>(mut self) -> Result<A, B>
    where
        Self: Sized,
        A: FromIterator<T>,
        B: FromIterator<E>,
    {
        self.by_ref()
            .collect::<Result<A, E>>()
            .map_err(|first_err| {
                std::iter::once(first_err)
                    .chain(self.filter_map(Result::err))
                    .collect()
            })
    }
}

impl<I, T, E> IteratorExt<T, E> for I where I: Iterator<Item = Result<T, E>> {}
