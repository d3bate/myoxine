//! Validates ASTs to ensure that they are well-formed.
use ast::ast::*;

/// Useful contextual information for checking that GraphQL asts are well-formed.
pub struct CheckingContext {
    /// The parsed schema which queries are validated against.
    schema: Document,
    /// This is needed to associate a `Span` with the output of error messages from
    /// this stage of doing things.
    attribute: syn::Attribute,
}

impl CheckingContext {
    /// Retrieves information about a type.
    fn retrieve_type_information<T>(type_name: T) -> TypeDefinition
    where
        T: AsRef<str>,
    {
        todo!()
    }
}

/// A trait for ensuring that ASTs are well-formed.
pub trait CheckAST<ERROR = syn::Error, CONTEXT = CheckingContext> {
    fn check(&self, context: CONTEXT) -> Result<(), ERROR>;
}

impl CheckAST for Document {
    fn check(&self, context: CheckingContext) -> Result<(), syn::Error> {
        todo!()
    }
}

impl CheckAST for Definition {
    fn check(&self, context: CheckingContext) -> Result<(), syn::Error> {
        todo!()
    }
}

impl CheckAST for ExecutableDefinition {
    fn check(&self, context: CheckingContext) -> Result<(), syn::Error> {
        todo!()
    }
}
