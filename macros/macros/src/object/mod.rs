/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

//! Contains code with which one can derive the `Object` trait on an item.

use ast::ast::{
    Definition, Document, GraphQLType, Name, ObjectTypeDefinition,
    TypeDefinition, TypeSystemDefinition,
};

use syn::export::Span;
use syn::DeriveInput;

const SCHEMA: &str = "schema";

/// Derives `Object` on the specified object. This function is probably going to take some
/// refinement and anyone willing to act as a guinea pig for it would be appreciated.
pub fn derive_object(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let schema_location = input
        .attrs
        .iter()
        .filter_map(|attribute| {
            let original_attribute = attribute;
            if let Ok(attribute) = attribute.parse_meta() {
                if let syn::Meta::NameValue(name_value) = attribute {
                    if name_value.path.is_ident(SCHEMA) {
                        if let syn::Lit::Str(string) = name_value.lit {
                            Some(Ok(string.value()))
                        } else {
                            Some(Err(syn::Error::new_spanned(
                                original_attribute,
                                "The type of `#[schema=<...>]` should be a string ",
                            )))
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
        .unwrap()?;
    let document = ast::parse_file(schema_location).map_err(|_| {
        syn::Error::new_spanned(
            input.ident.clone(),
            "The provided schema could not be parsed. Please ensure that your schema is
            valid, and if in doubt file a bug report at https://github.com/d3bate/myoxine.",
        )
    })?;
    let relevant_type = document.get_type(&Name(input.ident.to_string()));
    match relevant_type {
        Some(def) => match def {
            Definition::TypeSystemDefinition(def) => match def {
                TypeSystemDefinition::TypeDefinition(def) => match def {
                    ast::ast::TypeDefinition::ObjectTypeDefinition(object) => {
                        Ok(output_struct(object, &input)?)
                    }
                    _ => panic!("Invalid type."),
                },
                _ => panic!(),
            },
            _ => Err(syn::Error::new_spanned(
                input.ident.clone(),
                "Encountered an internal error.",
            )),
        },
        None => Err(syn::Error::new_spanned(
            input.ident.clone(),
            "This type could not be found in the schema.",
        )),
    }
}

/// Checks that the `Node` interface is defined on a type.
#[allow(dead_code)]
fn check_node_interface(document: &Document, _: &syn::Ident) -> Result<(), syn::Error> {
    document
        .0
        .iter()
        .filter_map(|definition| match definition {
            Definition::TypeSystemDefinition(def) => Some(def),
            _ => None,
        })
        .filter_map(|type_def| match type_def {
            TypeSystemDefinition::TypeDefinition(type_def) => Some(type_def),
            _ => None,
        })
        .filter_map(|type_def| match type_def {
            TypeDefinition::InterfaceTypeDefinition(def) => Some(def),
            _ => None,
        })
        .find(|interface| interface.name.0 == "Node".to_string())
        .map(|item| {
            if let Some(fields) = &item.fields_definition {
                if fields.0.len() > 1 {
                    Err(syn::Error::new(
                        Span::call_site(),
                        "Your `Node` interface has too many fields – it must have only \
                            one and it must be called `ID`!",
                    ))
                } else if let Some(item) = fields.0.get(0) {
                    if item.name.0 == "id".to_string()
                        && (item.graphql_type.extract_name().0).0 == "ID".to_string()
                        && (match item.graphql_type {
                            GraphQLType::NonNullType(_) => true,
                            _ => false,
                        })
                    {
                        Ok(item)
                    } else {
                        Err(syn::Error::new(
                            Span::call_site(),
                            "Your `Node` interface's `id` field is in some way malformed.",
                        ))
                    }
                } else {
                    Err(syn::Error::new(
                        Span::call_site(),
                        "This error shouldn't actually have \
                            happened; please do report it if it shows up :)",
                    ))
                }
            } else {
                Err(syn::Error::new(
                    Span::call_site(),
                    "Your `Node` interface doesn't have any \
                        fields. It should have one field `id` of type `ID!`",
                ))
            }
        });
    todo!()
}

fn output_struct(
    type_def: &ObjectTypeDefinition,
    input: &DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    check_type_def(type_def, input)?;
    let ident = input.ident.clone();
    let id_field = match input.data.clone() {
        syn::Data::Struct(s) => s
            .fields
            .iter()
            .filter(|field| {
                field
                    .attrs
                    .iter()
                    .filter(|attr| {
                        let x = match attr.parse_meta() {
                            Ok(m) => m,
                            Err(_) => return false,
                        };
                        if let syn::Meta::Path(p) = x {
                            p.is_ident("id")
                        } else {
                            return true;
                        }
                    })
                    .next()
                    .is_some()
            })
            .next()
            .map(Clone::clone)
            .expect("Missing ID field"),
        _ => panic!("Not a struct."),
    };
    #[allow(unused_variables)]
    let id_type = id_field.ty;
    let id_path = id_field.ident;
    let fields_type = quote::format_ident!("{}Fields", ident);
    let input_fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(n) => n,
            _ => panic!(),
        },
        _ => panic!(),
    }
    .named
    .iter()
    .map(|item| item.ident.as_ref().unwrap())
    .into_iter();
    Ok(quote::quote! {
        struct #fields_type {
            #(#input_fields: bool),*
        }
        impl ::myoxine::Object for #ident {
            type FieldsSelection = #fields_type;
            fn id(&self) -> ::myoxine ::Id {
                &self.#id_path
            }
            fn refetch_query(&self, fields: Self::FieldsSelection) -> ::myoxine::Query {
                ::myoxine::Query::new(format!("node(id: ) {\
                \
                }", self.id()))
            }
        }
    })
}

/// Maps built-in GraphQL types into the corresponding Rust ones. Note that Myoxine is picky about
/// which Rust types are valid for certain GraphQL types. These values have been chosen to match the
/// specification and reduce the possibility of errors arising.
///
/// This is an extremely rudimentary function and will need to be fleshed out in the future.
fn graphql2rust(input: &str) -> &str {
    match input {
        "Int" => "i32",
        "ID" => "String",
        _ => input,
    }
}

fn check_type_def(type_def: &ObjectTypeDefinition, input: &DeriveInput) -> Result<(), syn::Error> {
    let graphql_type_fields = type_def.fields_definition.clone().unwrap().0;
    match input.data.clone() {
        syn::Data::Struct(data_struct) => {
            for error in data_struct.fields.iter().map(|field| {
                let identifier = field.ident.clone().unwrap().to_string();
                if !match field.ty.clone() {
                    syn::Type::Path(path) => path.path.is_ident(graphql2rust(
                        &(graphql_type_fields
                            .iter()
                            .find(|field_definition| field_definition.name.0 == identifier)
                            .unwrap()
                            .graphql_type
                            .extract_name()
                            .0)
                            .0
                            .clone(),
                    )),
                    _ => panic!(),
                } {
                    Err(syn::Error::new_spanned(
                        field.ty.clone(),
                        "The type of this \
                    field does not match that of the GraphQL schema you have provided.",
                    ))
                } else {
                    Ok(())
                }
            }) {
                if let Err(e) = error {
                    return Err(e);
                }
            }
            Ok(())
        }
        _ => panic!("invalid type"),
    }
}

#[cfg(test)]
mod test_object_derive_macro {
    use super::*;

    #[test]
    fn test_simple_object_derivation() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
        #[derive(Query)]
        #[schema="schema.graphql"]
        struct User {
            #[id]
            id: i32,
            username: String
        }
        "#,
        )
        .expect("failed to parse");
        let output = derive_object(input).expect("failed to derive");
        assert!(
            crate::tests::token_streams_are_equal
                (output,
                 "impl :: myoxine :: Object for User { type Id = i32 ; fn id ( & self ) -> & Self :: Id
                { & self . id } fn refetch_query ( & self ) -> :: myoxine :: Query { todo ! ( ) } }"
                     .parse::<proc_macro2::TokenStream>()
                     .unwrap(),
                )
        );
    }

    #[test]
    fn test_more_complex_object_derivation() {
        todo!()
    }

    #[test]
    fn test_derivation_with_other_objects() {
        todo!()
    }
}
