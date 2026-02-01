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

use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    marker::PhantomData,
    ops::Deref,
    panic::Location,
};

/// Exception type that can hold an error tree and additional context
pub struct Exn<E: Error + Send + Sync + 'static> {
    frame: Box<Frame>,
    phantom: PhantomData<E>,
}

impl<E: Error + Send + Sync + 'static> Exn<E> {
    /// Creates a new [`Exn`] with the given `error`.
    ///
    /// The types of all [source `Error`s] are erased, but their [`String`] representations are
    /// retained.
    ///
    /// See [`ErrorExt::raise`] for a fluent way to convert an error into an [`Exn`].
    ///
    /// [source `Error`s]: Error::source
    /// [`ErrorExt::raise`]: crate::ErrorExt::raise
    #[track_caller]
    pub fn new(error: E) -> Self {
        struct SourceError(String);

        impl Debug for SourceError {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Debug::fmt(&self.0, f)
            }
        }

        impl Display for SourceError {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl Error for SourceError {}

        fn walk(error: &dyn Error, location: &'static Location<'static>) -> Vec<Frame> {
            error
                .source()
                .map(|source| Frame {
                    error: Box::new(SourceError(source.to_string())),
                    location,
                    children: walk(source, location),
                })
                .into_iter()
                .collect()
        }

        let location = Location::caller();
        let children = walk(&error, location);
        let frame = Frame {
            error: Box::new(error),
            location,
            children,
        };

        Self {
            frame: Box::new(frame),
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Exn`] with the given `error` and its `children`.
    #[track_caller]
    pub fn raise_all<T, I>(children: I, error: E) -> Self
    where
        T: Error + Send + Sync + 'static,
        I: IntoIterator,
        I::Item: Into<Exn<T>>,
    {
        let mut new_exn = Exn::new(error);
        // Note: We can't use `Vec::extend` since `#[track_caller]` on closures is currently
        // unstable.
        for child in children {
            new_exn.frame.children.push(*child.into().frame);
        }
        new_exn
    }

    /// Creates a new [`Exn`] where `self` is its child.
    #[track_caller]
    pub fn raise<T: Error + Send + Sync + 'static>(self, error: T) -> Exn<T> {
        let mut new_exn = Exn::new(error);
        new_exn.frame.children.push(*self.frame);
        new_exn
    }

    /// Returns a reference to the underlying exception frame.
    #[must_use]
    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    /// Converts this [`Exn`] into its underlying exception frame.
    #[deprecated(since = "0.3.1", note = "Use `Frame::from` instead")]
    #[must_use]
    pub fn into_frame(self) -> Frame {
        *self.frame
    }
}

impl<E: Error + Send + Sync + 'static> Deref for Exn<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.frame
            .error()
            .downcast_ref()
            .unwrap_or_else(|| unreachable!("error type must match"))
    }
}

impl<E: Error + Send + Sync + 'static> Debug for Exn<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("Exn")
                .field("frame", self.frame())
                .finish_non_exhaustive()
        } else {
            self.frame.debug_full(f)
        }
    }
}

impl<E: Error + Send + Sync + 'static> Display for Exn<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<T, E> From<T> for Exn<E>
where
    T: Error + Into<E>,
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn from(error: T) -> Self {
        Exn::new(error.into())
    }
}

/// Node in an exception tree
pub struct Frame {
    /// Originating error
    error: Box<dyn Error + Send + Sync + 'static>,

    /// Source location where this frame was created
    location: &'static Location<'static>,

    /// Child frames that provide additional context or source error information
    children: Vec<Frame>,
}

impl Frame {
    /// Returns the [`Error`] that occurred at this frame.
    #[must_use]
    pub fn error(&self) -> &(dyn Error + Send + Sync + 'static) {
        &*self.error
    }

    /// Returns the source location where this frame was created.
    #[must_use]
    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }

    /// Returns all child [`Frame`]s.
    #[must_use]
    pub fn children(&self) -> &[Frame] {
        &self.children
    }

    /// Converts this [`Frame`] into its error and its children.
    #[must_use]
    pub fn consume(self) -> (Box<dyn Error + Send + Sync + 'static>, Vec<Self>) {
        (self.error, self.children)
    }

    /// Performs standard [`Debug`] formatting for only this [`Frame`] (i.e., excluding children).
    #[expect(clippy::missing_errors_doc, reason = "fmt::Result")]
    pub fn debug(&self, f: &mut Formatter) -> fmt::Result {
        let location = self.location();
        write!(
            f,
            "{}, at {}:{}:{}",
            self.error(),
            location.file(),
            location.line(),
            location.column()
        )
    }

    /// Performs standard [`Debug`] formatting for this [`Frame`] and its children recursively.
    #[expect(clippy::missing_errors_doc, reason = "fmt::Result")]
    pub fn debug_full(&self, f: &mut Formatter) -> fmt::Result {
        self.debug_recursive(f, true, "")
    }

    fn debug_recursive(&self, f: &mut Formatter, root: bool, prefix: &str) -> fmt::Result {
        self.debug(f)?;

        let children = self.children();
        let children_len = children.len();

        for (i, child) in children.iter().enumerate() {
            let child_children_len = child.children().len();
            if root && children_len == 1 && child_children_len == 1 {
                // Flatten chains of single children to minimize indentation.
                write!(f, "\n{prefix}├─ ")?;
                child.debug_recursive(f, root, prefix)?;
            } else if i < children_len - 1 {
                write!(f, "\n{prefix}├─ ")?;
                child.debug_recursive(f, false, &format!("{prefix}│  "))?;
            } else {
                write!(f, "\n{prefix}└─ ")?;
                child.debug_recursive(f, false, &format!("{prefix}   "))?;
            }
        }

        Ok(())
    }
}

impl Debug for Frame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("Frame")
                .field("error", self.error())
                .field("location", self.location)
                .field("children", &self.children)
                .finish()
        } else {
            self.debug(f)
        }
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.error(), f)
    }
}

impl Error for Frame {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.children
            .first()
            .map(|child| child as &(dyn Error + 'static))
    }
}

impl<E: Error + Send + Sync + 'static> From<Exn<E>> for Frame {
    fn from(exn: Exn<E>) -> Self {
        *exn.frame
    }
}
