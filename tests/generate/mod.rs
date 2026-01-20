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

use exn::{ErrorExt, Exn};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(pub &'static str);

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ErrorWithSource(pub &'static str, #[source] pub Error);

pub fn tree() -> Exn<Error> {
    let e1 = Error("E1").raise();
    let e3 = e1.raise(Error("E3"));

    let e9 = Error("E9").raise();
    let e10 = e9.raise(Error("E10"));

    let e11 = Error("E11").raise();
    let e12 = e11.raise(Error("E12"));

    let e5 = Exn::raise_all([e3, e10, e12], Error("E5"));

    let e2 = Error("E2").raise();
    let e4 = e2.raise(Error("E4"));

    let e7 = Error("E7").raise();
    let e8 = e7.raise(Error("E8"));

    Exn::raise_all([e5, e4, e8], Error("E6"))
}

pub fn linear() -> Exn<Error> {
    let e1 = Error("E1").raise();
    let e2 = e1.raise(Error("E2"));
    let e3 = e2.raise(Error("E3"));
    let e4 = e3.raise(Error("E4"));
    e4.raise(Error("E5"))
}
