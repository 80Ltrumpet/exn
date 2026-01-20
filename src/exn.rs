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

    /// Creates a new exception with the given `error` and its `children`.
    #[track_caller]
    pub fn raise_all<T, I>(error: E, children: I) -> Self
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

    /// Raise a new exception; this will make the current exception a child of the new one.
    #[track_caller]
    pub fn raise<T: Error + Send + Sync + 'static>(self, err: T) -> Exn<T> {
        let mut new_exn = Exn::new(err);
        new_exn.frame.children.push(*self.frame);
        new_exn
    }

    /// Return the underlying exception frame.
    #[must_use]
    pub fn frame(&self) -> &Frame {
        &self.frame
    }
}

impl<E> Deref for Exn<E>
where
    E: Error + Send + Sync + 'static,
{
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
        self.frame().debug(f)
    }
}

impl<E: Error + Send + Sync + 'static> Display for Exn<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<E: Error + Send + Sync + 'static> From<E> for Exn<E> {
    #[track_caller]
    fn from(error: E) -> Self {
        Exn::new(error)
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

    fn debug(&self, f: &mut Formatter) -> fmt::Result {
        self.debug_recursive(f, true, "")
    }

    fn debug_recursive(&self, f: &mut Formatter, root: bool, prefix: &str) -> fmt::Result {
        {
            let location = self.location();
            write!(
                f,
                "{}, at {}:{}:{}",
                self.error(),
                location.file(),
                location.line(),
                location.column()
            )?;
        }

        let children = self.children();
        let children_len = children.len();

        for (i, child) in children.iter().enumerate() {
            let child_children_len = child.children().len();
            if root && children_len == 1 && child_children_len == 1 {
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
        self.debug(f)
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.error(), f)
    }
}
