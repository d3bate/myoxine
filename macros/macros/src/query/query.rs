/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/

//! Implements the `Query` trait on items.
//!
//! This macros currently only handles a subset of the GraphQL specification, so some things might
//! not work. If you need a feature which isn't yet available there are two options:
//! 1. Implement it yourself (and then submit a pull request to the project)
//! 2. Use a supported feature and submit an issue. We do intend to add complete support for all of
//! the specification so there's a good chance it will be implemented.
//!
//! One nice thing about this part of the codebase is that the entire API is private, so the churn
//! can be pretty high without causing issues.

use ast::ast::*;
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub struct QueryCodegenMeta {
    derive_input: syn::DeriveInput,
}

impl QueryCodegenMeta {
    /// Retrieve the text of the query, if possible.
    ///
    /// This might return an error if the `#[query=<x>]` attribute has not been defined on the
    /// struct or the type of the literal is not correct (should be a string).
    fn get_query(&self) -> Result<String, syn::Error> {
        self.derive_input
            .attrs
            .iter()
            .find(|item| {
                match item.parse_meta().map_err(|_| false).map(|meta| match meta {
                    syn::Meta::NameValue(name_value) => {
                        if name_value.path.is_ident("query") {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }) {
                    Ok(t) => t,
                    Err(e) => e,
                }
            })
            .map(|item| match item.parse_meta().unwrap() {
                syn::Meta::NameValue(nv) => match nv.lit {
                    syn::Lit::Str(string) => Ok(string.value()),
                    _ => Err(syn::Error::new_spanned(
                        item.tokens.clone(),
                        "The value of the `#[query=<x>]` attribute should be a string.",
                    )),
                },
                _ => panic!(),
            })
            // at some point this should be replaced with some code to return a proper error message
            // yes writing this comment probably took more time than the code to return that error
            // message would require
            .unwrap()
    }
}

/// A trait to generate output the Rust code needed for a query.
pub trait QueryCodegen<META = QueryCodegenMeta> {
    fn output(&self, meta: &META) -> Result<TokenStream, syn::Error>;
}

impl QueryCodegen for Document {
    fn output(&self, meta: &QueryCodegenMeta) -> Result<TokenStream, syn::Error> {
        let struct_identifier = meta.derive_input.ident.clone();
        let literal = meta.get_query()?;
        Ok(quote::quote! {
            impl Query for #struct_identifier {
                fn query() -> String {
                    #literal
                }
            }
        })
    }
}

impl QueryCodegen for Definition {
    fn output(&self, meta: &QueryCodegenMeta) -> Result<TokenStream, syn::Error> {
        todo!()
    }
}

impl QueryCodegen for ExecutableDefinition {
    fn output(&self, meta: &QueryCodegenMeta) -> Result<TokenStream, syn::Error> {
        todo!()
    }
}

impl QueryCodegen for OperationDefinition {
    fn output(&self, meta: &QueryCodegenMeta) -> Result<TokenStream, syn::Error> {
        todo!()
    }
}

pub fn query_inner(input: DeriveInput) -> TokenStream {
    todo!()
}
