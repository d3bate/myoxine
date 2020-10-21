/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/
//! Useful macros for creating GraphQL queries inside applications.

mod check;
mod mutation;
mod object;
mod query;
mod search;

mod tests;

extern crate proc_macro;

use crate::query::query_inner;
use proc_macro::TokenStream;

#[proc_macro_derive(Object, attributes(schema, id))]
pub fn derive_object_on_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match object::derive_object(input) {
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
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
