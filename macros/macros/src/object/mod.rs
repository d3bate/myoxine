/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

//! Contains code with which one can derive the `Object` trait on an item.

use ast::ast::{Definition, Name, ObjectTypeDefinition, TypeDefinition, TypeSystemDefinition};

use syn::DeriveInput;

const SCHEMA: &str = "schema";

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
                        if let syn::Meta::NameValue(name_value) = x {
                            if name_value.path.is_ident("id") {
                                if let syn::Lit::Str(string) = name_value.lit {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
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
    let id_type = id_field.ty;
    let id_path = id_field.ident;
    quote::quote! {
        impl ::myoxine::Object for #ident {
            type Id = #id_type;
            fn id(&self) -> &Self::Id {
                self.#id_path
            }
            fn refetch_query(&self) -> ::myoxine::Query {
                todo!()
            }
        }
    };
    todo!()
}

fn check_type_def(type_def: &ObjectTypeDefinition, input: &DeriveInput) -> Result<(), syn::Error> {
    let graphql_type_fields = type_def.fields_definition.clone().unwrap().0;
    match input.data.clone() {
        syn::Data::Struct(data_struct) => {
            for error in data_struct.fields.iter().map(|field| {
                let identifier = field.ident.clone().unwrap().to_string();
                if !match field.ty.clone() {
                    syn::Type::Path(path) => path.path.is_ident(
                        &(graphql_type_fields
                            .iter()
                            .find(|field_definition| field_definition.name.0 == identifier)
                            .unwrap()
                            .graphql_type
                            .extract_name()
                            .0)
                            .0
                            .clone(),
                    ),
                    _ => panic!(),
                } {
                    Err(syn::Error::new_spanned(field.ty.clone(), "The type of this field does not match that of the GraphQL schema you have provided."))
                } else {
                    Ok(())
                }
            }) {
                if let Err(e) = error {
                    return Err(e);
                }
            };
            Ok(())
        }
        _ => panic!("invalid type"),
    }
}
