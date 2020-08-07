use proc_macro2::TokenStream;

/// The query macro. Note that this is named `query_inner` because it takes types from the
/// `proc_macro2` crate rather than the `proc_macro` crate. This is useful for testing the macro.
#[allow(dead_code)]
pub fn query_inner(_input: TokenStream) -> TokenStream {
    todo!()
}
