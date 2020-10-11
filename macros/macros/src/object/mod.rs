/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

//! Contains code with which one can derive the `Object` trait on an item.

use quote::ToTokens;
use syn::parse::Parse;
pub struct ObjectDeriveInput {
    derive_input: syn::DeriveInput,
}

impl Parse for ObjectDeriveInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            derive_input: input.parse()?,
        })
    }
}

impl ToTokens for ObjectDeriveInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.derive_input.ident;
        let output = quote::quote! {
            impl Object for #ident {
                fn refetch_query() {

                }
            }
        };
        tokens.extend(output);
    }
}
