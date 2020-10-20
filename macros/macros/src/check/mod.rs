/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

//! Validates ASTs to ensure that they are well-formed.
//!
//! Note that this is presently a work in progress and expect this file to change as time goes on.

use ast::ast::*;

/// Useful contextual information for checking that GraphQL asts are well-formed.
pub struct QueryCheckingContext {
    /// The parsed schema which queries are validated against.
    schema: Document,
    /// This is needed to associate a `Span` with the output of error messages from
    /// this stage of doing things.
    attribute: syn::Attribute,
}

impl QueryCheckingContext {
    /// Retrieves information about a type.
    fn retrieve_type_information<T>(type_name: T) -> TypeDefinition
    where
        T: AsRef<str>,
    {
        todo!()
    }
}

fn call_site_spanned_error(error: &'static str) -> syn::Error {
    syn::Error::new(proc_macro2::Span::call_site(), error)
}

/// A trait for ensuring that ASTs are well-formed queries.
pub trait CheckQuery<ERROR = syn::Error, CONTEXT = QueryCheckingContext> {
    fn check(&self, context: &CONTEXT) -> Result<(), ERROR>;
}

impl CheckQuery for Document {
    fn check(&self, context: &QueryCheckingContext) -> Result<(), syn::Error> {
        for definition in &self.0 {
            definition.check(context)?;
        }
        Ok(())
    }
}

impl CheckQuery for Definition {
    fn check(&self, context: &QueryCheckingContext) -> Result<(), syn::Error> {
        match self {
            Definition::ExecutableDefinition(def) => def.check(context),
            Definition::TypeSystemDefinition(_) => Err(call_site_spanned_error(
                "Type system definitions are not valid inside Myoxine queries.",
            )),
            Definition::TypeSystemExtension(_) => Err(call_site_spanned_error(
                "Type system definitions are not valid inside Myoxine queries.",
            )),
        }
    }
}

impl CheckQuery for ExecutableDefinition {
    fn check(&self, context: &QueryCheckingContext) -> Result<(), syn::Error> {
        match self {
            ExecutableDefinition::OperationDefinition(op_def) => op_def.check(context),
            ExecutableDefinition::FragmentDefinition(_) => Err(call_site_spanned_error(
                "Fragments are not yet supported, though support is planned.",
            )),
        }
    }
}

impl CheckQuery for OperationDefinition {
    fn check(&self, context: &QueryCheckingContext) -> Result<(), syn::Error> {
        match self.operation_type.token {
            OperationType::Query => {}
            OperationType::Subscription | OperationType::Mutation => {
                return Err(call_site_spanned_error("Mutations and subscriptions are not permitted inside queries.
                           Subscriptions are also not currently supported, though support is on the long-term roadmap."))
            }
        };
        todo!()
    }
}
