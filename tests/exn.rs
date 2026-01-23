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

mod generate;

use exn::{Exn, OptionExt, Result, ResultExt, repr};

use self::generate::Error;

#[test]
fn list_repr_tree() {
    let e = generate::list::<repr::Tree>().unwrap_err();
    insta::assert_compact_debug_snapshot!(e);
}

#[test]
fn list_repr_list() {
    let e = generate::list::<repr::List>().unwrap_err();
    insta::assert_compact_debug_snapshot!(e);
}

#[test]
fn tree_repr_tree() {
    let e = generate::tree::<repr::Tree>().unwrap_err();
    insta::assert_compact_debug_snapshot!(e);
}

#[test]
fn tree_repr_list() {
    let e = generate::tree::<repr::List>().unwrap_err();
    insta::assert_compact_debug_snapshot!(e);
}

#[test]
fn new_with_source() {
    #[derive(Debug, thiserror::Error)]
    #[error("{0}")]
    struct ErrorWithSource(&'static str, #[source] Error);

    let e = Exn::new(ErrorWithSource("top", Error("source")));
    insta::assert_compact_debug_snapshot!(e);
}

#[test]
fn result_ext() {
    let result = Err::<(), _>(Error("An error"));
    let result: Result<(), Error> = result.or_raise(|| Error("Another error"));
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn option_ext() {
    let result = None::<()>;
    let result: Result<(), Error> = result.ok_or_raise(|| Error("An error"));
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn from_error() {
    fn foo() -> Result<(), Error> {
        Err(Error("An error"))?;
        Ok(())
    }

    let result = foo();
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn bail() {
    fn foo() -> Result<(), Error> {
        exn::bail!(Error("An error"));
    }

    let result = foo();
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn ensure_ok() {
    fn foo() -> Result<(), Error> {
        exn::ensure!(true, Error("An error"));
        Ok(())
    }

    foo().unwrap();
}

#[test]
fn ensure_fail() {
    fn foo() -> Result<(), Error> {
        exn::ensure!(false, Error("An error"));
        Ok(())
    }

    let result = foo();
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}
