use proc_macro2::TokenStream;

/// The query macro. Note that this is named `query_inner` because it takes types from the
/// `proc_macro2` crate rather than the `proc_macro` crate. This is useful for testing the macro.
pub fn query_inner(input: TokenStream) -> TokenStream {
    todo!()
}
