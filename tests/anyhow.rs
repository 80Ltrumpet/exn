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

mod generate;

use anyhow::Context;
use exn::repr::{Anyhow, List};

#[test]
fn list_repr_tree() {
    let result = generate::list::<Anyhow>().context("context");
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn list_repr_list() {
    let result = generate::list::<Anyhow<List>>().context("context");
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn tree_repr_tree() {
    let result = generate::tree::<Anyhow>().context("context");
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}

#[test]
fn tree_repr_list() {
    let result = generate::tree::<Anyhow<List>>().context("context");
    insta::assert_compact_debug_snapshot!(result.unwrap_err());
}
