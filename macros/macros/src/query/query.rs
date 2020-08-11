/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/

//! Parses Rust GraphQL queries.
//!
//! These are then turned into optimised static queries to be executed at compile time. In the
//! process of this these queries are also type checked.
//!
//! The process is something like:
//! 1. parse query
//! 2. check that the query is valid in the context of the schema
//!    (i) note that the parsed schema is cached in a folder called `.myoxine` to speed up compile
//!        times as much as is possible.
//! 3. output the runtime code for the query
//!
//! In the future we might also include a derive macro for the `Query` trait.
use syn::parse::{Parse, ParseStream};
use syn::Ident;
use syn::Result;
use syn::Token;

/// A struct on which `Query` is to be implemented.
pub struct QueryInput {
    /// The name of the struct to be created.
    output_name: Ident,
    /// The AST representing the query.
    query: Query,
}

impl Parse for QueryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            output_name: input.parse::<syn::Ident>()?,
            query: input.parse::<Query>()?,
        })
    }
}

/// The query
pub struct Query {
    /// Two curly brackets surrounding the content.
    brace_token: syn::token::Brace,
    /// The selection to be made.
    selection: SelectionSet,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> Result<Self> {
        let brace;
        Ok(Self {
            brace_token: syn::braced!(brace in input),
            selection: input.parse::<SelectionSet>()?,
        })
    }
}

/// A selection set.
pub struct SelectionSet {
    selection_list: Selections,
}

impl Parse for SelectionSet {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            selection_list: input.parse::<Selections>()?,
        })
    }
}

/// A list of all the selections in a selection set.
pub struct Selections(pub Vec<Selection>);

impl Parse for Selections {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut output = vec![];
        while !input.is_empty() {
            output.push(input.parse::<Selection>()?)
        }
        Ok(Self(output))
    }
}

/// A member of a selection set.
pub enum Selection {
    Field(Field),
    FragmentSpread,
    InlineFragment,
}

impl Parse for Selection {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}

/// A GraphQL field.
pub struct Field {
    alias: Option<Alias>,
    name: Ident,
    selection_set: Option<SelectionSet>,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}

/// A GraphQL Alias
pub struct Alias {
    name: Ident,
    colon: syn::Token![:],
}

impl Parse for Alias {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}
