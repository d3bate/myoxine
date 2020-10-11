/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

//! Contains an AST for a GraphQL schema. This is used to conduct operations on externally defined
//! files.
//!
//! As is our approach to all documentation in this project – if you don't understand something,
//! please do ask! That way we can improve the documentation for everyone and make it easier to
//! understand.

#[macro_use]
extern crate pest_derive;
use crate::ast::{Document, GraphQLParser};
use pest::Parser;
use std::convert::TryFrom;
use std::fs::read_to_string;
use std::path::Path;

pub mod ast;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ParseFileError {
    #[error("couldn't read from the supplied file")]
    FileOpenError(std::io::Error),
    #[error("couldn't parse the supplied file")]
    ParseError(pest::error::Error<ast::Rule>),
}

/// Parse a schema from a provided file.
///
/// Schemas must be contained in a single source file.
///
/// If you already have a GraphQL server you can automatically generate a schema, without the need
/// to create one by hand. This is strongly advised as it will save you a lot of time.
pub fn parse_file<P>(path: P) -> std::result::Result<ast::Document, ParseFileError>
where
    P: AsRef<Path>,
{
    let string = match read_to_string(path) {
        Ok(string) => string,
        Err(e) => return Err(ParseFileError::FileOpenError(e)),
    };
    parse_string(string).map_err(|error| ParseFileError::ParseError(error))
}

pub fn parse_string<P>(
    string: P,
) -> std::result::Result<ast::Document, pest::error::Error<ast::Rule>>
where
    P: AsRef<str>,
{
    let parsed = GraphQLParser::parse(ast::Rule::document, string.as_ref())?
        .next()
        .unwrap();

    Document::try_from(parsed)
}
