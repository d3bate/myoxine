/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/
//! Useful macros for creating GraphQL queries inside applications.

mod check;
mod mutation;
mod query;

extern crate proc_macro;

use crate::query::query_inner;
use proc_macro::TokenStream;

#[proc_macro_derive(Query)]
/// A derive macro which implements the `Query` trait on structs.
pub fn query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    TokenStream::from(query_inner(input.into()))
}

#[proc_macro]
/// A macros for generating GraphQL mutations.
/// ```ignore
/// use macros::mutation;
/// let user_id = 1;
/// let value = "Some Value".to_string(); // needs to be able to take ownership of this
///                                       // i.e. must be `Clone` or `Copy` or not referenced
///                                       // later in the program.
/// mutation! {
///     updateUserById(id: user_id, value: value) {
///         id,
///         username,
///         email
///     }
/// }
/// ```
pub fn mutation(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
