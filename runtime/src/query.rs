/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/

//! Represents a query.
//!
//! This file might be a bit confusing, because it uses a lot of type system magic to try and embed
//! GraphQL inside of Rust, which is possible (because Rust's type system is turing-complete) but
//! makes it harder to use if you're not familiar with a programming language with a heavy emphasis
//! on types (e.g. Haskell or OCaml). Don't worry, though, for we're trying to explain everything
//! thoroughly.

#[derive(Default)]
/// A struct wrapping a `String`.
pub struct Query(String);

impl Query {
    pub fn new(query: String) -> Self {
        Self(query)
    }
}
