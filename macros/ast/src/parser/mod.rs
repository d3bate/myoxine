/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/

//! Parses GraphQL schema files.
//!
//! This is useful for compile-time type checking. You might also find this ast useful in other
//! projects, but if you're interested in speed we'd suggest you use a different ast.
//!
//! If you're unsure about any of the code in this file, please do ask about it!

use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair};
use std::convert::TryFrom;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[grammar = "graphql.pest"]
pub struct GraphQLParser;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Name(String);

impl<'a> TryFrom<Pair<'a, Rule>> for Name {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(pair.as_str().to_string()))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

impl<'a> TryFrom<Pair<'a, Rule>> for OperationType {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_str() {
            "query" => Ok(Self::Query),
            "mutation" => Ok(Self::Mutation),
            "subscription" => Ok(Self::Subscription),
            _ => panic!(
                "Internal ast error. Please report this to https://github.com/d3bate/myoxine"
            ),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct NamedType(pub String);

impl<'a> TryFrom<Pair<'a, Rule>> for NamedType {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(pair.as_str().to_string()))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct RootOperationTypeDefinition {
    /// The name of the operation (either "schema", "mutation" or "subscription").
    /// NOTE: GraphQL subscriptions are not currently supported, although support is planned.
    pub operation_type: OperationType,
    /// The type which this operation refers to.
    pub named_type: NamedType,
}

impl<'a> TryFrom<Pair<'a, Rule>> for RootOperationTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut inner = pair.into_inner();
        Ok(Self {
            operation_type: OperationType::try_from(inner.next().unwrap())?,
            named_type: NamedType::try_from(inner.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Argument {
    name: Name,
    value: Value,
}

impl<'a> TryFrom<Pair<'a, Rule>> for Argument {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            name: Name::try_from(iterator.next().unwrap())?,
            value: Value::try_from(iterator.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Arguments(pub Vec<Argument>);

impl<'a> TryFrom<Pair<'a, Rule>> for Arguments {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let arguments = pair.into_inner();
        let mut output = vec![];
        for argument in arguments {
            output.push(Argument::try_from(argument)?);
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Directive {
    name: Name,
    arguments: Option<Arguments>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for Directive {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            name: Name::try_from(iterator.next().unwrap())?,
            arguments: match iterator.next() {
                Some(pair) => Some(Arguments::try_from(pair)?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Directives(Vec<Directive>);

impl<'a> TryFrom<Pair<'a, Rule>> for Directives {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut result = vec![];
        for item in iterator {
            result.push(Directive::try_from(item)?)
        }
        Ok(Self(result))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SchemaDefinition {
    /// Describes the schema.
    description: Option<Description>,
    /// Directives
    directives: Option<Directives>,
    /// Queries which can be used to retrieve data from the server.
    query: Option<RootOperationTypeDefinition>,
    /// Mutations with which data can be updated on the server.
    mutation: Option<RootOperationTypeDefinition>,
    /// This isn't supported and is ignored.
    subscription: Option<RootOperationTypeDefinition>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for SchemaDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut schema_definition = Self::default();
        let mut iterator = pair.into_inner();
        let optional_description = iterator.peek().unwrap();
        match optional_description.as_rule() {
            Rule::description => {
                schema_definition.description = Some(Description::try_from(optional_description)?);
                iterator.next().unwrap();
            }
            Rule::root_operation_type_definition | Rule::directives => {}
            _ => unreachable!(),
        };

        let optional_directives = iterator.peek().unwrap();
        match optional_directives.as_rule() {
            Rule::directives => {
                schema_definition.directives = Some(Directives::try_from(optional_directives)?);
            }
            Rule::root_operation_type_definition => {}
            _ => unreachable!(),
        }

        while let Some(field) = iterator.next() {
            let root_operation = RootOperationTypeDefinition::try_from(field.clone())?;
            match root_operation.operation_type {
                OperationType::Subscription => {
                    if schema_definition.subscription.is_some() {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "The `subscription` field has been defined twice."
                                    .to_string(),
                            },
                            field.as_span(),
                        ));
                    }
                    schema_definition.subscription = Some(root_operation);
                }
                OperationType::Query => {
                    if schema_definition.query.is_some() {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "The `query` field has been defined twice.".to_string(),
                            },
                            field.as_span(),
                        ));
                    }
                    schema_definition.query = Some(root_operation);
                }
                OperationType::Mutation => {
                    if schema_definition.mutation.is_some() {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "The `mutation` field has been defined twice.".to_string(),
                            },
                            field.as_span(),
                        ));
                    }
                    schema_definition.mutation = Some(root_operation);
                }
            }
        }

        Ok(schema_definition)
    }
}

impl Default for SchemaDefinition {
    fn default() -> Self {
        Self {
            description: None,
            directives: None,
            query: None,
            mutation: None,
            subscription: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum TypeDefinition {
    ScalarTypeDefinition(ScalarTypeDefinition),
    ObjectTypeDefinition(ObjectTypeDefinition),
    InterfaceTypeDefinition(InterfaceTypeDefinition),
    UnionTypeDefinition(UnionTypeDefinition),
    EnumTypeDefinition(EnumTypeDefinition),
    InputObjectTypeDefinition(InputObjectTypeDefinition),
}

impl<'a> TryFrom<Pair<'a, Rule>> for TypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut pair = pair.into_inner();
        let definition = pair.next().unwrap();
        match definition.as_rule() {
            Rule::scalar_type_definition => Ok(Self::ScalarTypeDefinition(
                ScalarTypeDefinition::try_from(definition)?,
            )),
            Rule::object_type_definition => Ok(Self::ObjectTypeDefinition(
                ObjectTypeDefinition::try_from(definition)?,
            )),
            Rule::interface_type_definition => Ok(Self::InterfaceTypeDefinition(
                InterfaceTypeDefinition::try_from(definition)?,
            )),
            Rule::union_type_definition => Ok(Self::UnionTypeDefinition(
                UnionTypeDefinition::try_from(definition)?,
            )),
            Rule::enum_type_definition => Ok(Self::EnumTypeDefinition(
                EnumTypeDefinition::try_from(definition)?,
            )),
            Rule::input_object_type_definition => Ok(Self::InputObjectTypeDefinition(
                InputObjectTypeDefinition::try_from(definition)?,
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ScalarTypeDefinition {
    description: Option<Description>,
    name: Name,
    directives: Option<Directives>,
}

impl Default for ScalarTypeDefinition {
    fn default() -> Self {
        Self {
            description: None,
            name: Name("".to_string()),
            directives: None,
        }
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for ScalarTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut scalar_type_definition = ScalarTypeDefinition::default();
        let mut iterator = pair.into_inner();
        let possible_description = iterator.peek().unwrap();
        match possible_description.as_rule() {
            Rule::description => {
                scalar_type_definition.description =
                    Some(Description::try_from(possible_description)?);
                iterator.next().unwrap();
            }
            _ => {}
        }
        scalar_type_definition.name = Name::try_from(iterator.next().unwrap())?;
        match iterator.next() {
            Some(t) => {
                scalar_type_definition.directives = Some(Directives::try_from(t)?);
            }
            None => {}
        }
        Ok(scalar_type_definition)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ScalarTypeExtension {
    name: Name,
    directives: Option<Directives>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for ScalarTypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        // skip "extend scalar" token
        iterator.next().unwrap();
        Ok(ScalarTypeExtension {
            name: Name::try_from(iterator.next().unwrap())?,
            directives: match iterator.next() {
                Some(item) => Some(Directives::try_from(item)?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ImplementsInterfaces(Vec<NamedType>);

impl<'a> TryFrom<Pair<'a, Rule>> for ImplementsInterfaces {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut output = vec![];
        for item in pair.into_inner() {
            output.push(NamedType::try_from(item)?);
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Description(pub String);

impl<'a> TryFrom<Pair<'a, Rule>> for Description {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(pair.into_inner().as_str().to_string()))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum GraphQLType {
    NamedType(NamedType),
    ListType(Box<GraphQLType>),
    NonNullType(Box<GraphQLType>),
}

impl<'a> TryFrom<Pair<'a, Rule>> for GraphQLType {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let graphql_type = pair.into_inner().next().unwrap();
        match graphql_type.as_rule() {
            Rule::named_type => Ok(Self::NamedType(NamedType::try_from(graphql_type)?)),
            Rule::list_type => Ok(Self::ListType(Box::new(Self::try_from(graphql_type)?))),
            Rule::non_null_type => Ok(Self::NonNullType(Box::new(Self::try_from({
                graphql_type
            })?))),
            // ^^ maybe not 100% compliant with the spec
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ArgumentsDefinition(pub Vec<InputValueDefinition>);

impl<'a> TryFrom<Pair<'a, Rule>> for ArgumentsDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut output = vec![];
        for token in iterator {
            output.push(InputValueDefinition::try_from(token)?)
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Alias {
    name: Name,
}

impl<'a> TryFrom<Pair<'a, Rule>> for Alias {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: Name::try_from(pair.into_inner().next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Field {
    alias: Option<Alias>,
    name: Name,
    variable_definitions: Option<VariableDefinitions>,
    directives: Option<Directives>,
    selection_set: Option<SelectionSet>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for Field {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            alias: match iterator.peek().unwrap().as_rule() {
                Rule::alias => Some(Alias::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            name: Name::try_from(iterator.next().unwrap())?,
            variable_definitions: match iterator.peek() {
                Some(item) => match item.as_rule() {
                    Rule::variable_definitions => {
                        Some(VariableDefinitions::try_from(iterator.next().unwrap())?)
                    }
                    _ => None,
                },
                None => None,
            },
            directives: match iterator.peek() {
                Some(item) => match item.as_rule() {
                    Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
                    _ => None,
                },
                None => None,
            },
            selection_set: match iterator.peek() {
                Some(item) => match item.as_rule() {
                    Rule::selection_set => Some(SelectionSet::try_from(iterator.next().unwrap())?),
                    _ => None,
                },
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct FieldDefinition {
    description: Option<Description>,
    name: Name,
    arguments_definition: Option<ArgumentsDefinition>,
    graphql_type: GraphQLType,
    directives: Option<Directives>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for FieldDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: match iterator.peek().unwrap().as_rule() {
                Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            name: Name::try_from(iterator.next().unwrap())?,
            arguments_definition: match iterator.peek().unwrap().as_rule() {
                Rule::arguments_definition => {
                    Some(ArgumentsDefinition::try_from(iterator.next().unwrap())?)
                }
                _ => None,
            },
            graphql_type: GraphQLType::try_from(iterator.next().unwrap())?,
            directives: match iterator.peek() {
                Some(directives) => Some(Directives::try_from(directives)?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct FieldsDefinition(pub Vec<FieldDefinition>);

impl<'a> TryFrom<Pair<'a, Rule>> for FieldsDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut output = vec![];
        for token in pair.into_inner() {
            output.push(FieldDefinition::try_from(token)?)
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ObjectTypeDefinition {
    description: Option<Description>,
    name: Name,
    implements_interfaces: Option<ImplementsInterfaces>,
    directives: Option<Directives>,
    fields_definition: Option<FieldsDefinition>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for ObjectTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: {
                let next = iterator.peek().unwrap();
                match next.as_rule() {
                    Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                    _ => None,
                }
            },
            name: Name::try_from(iterator.next().unwrap())?,
            implements_interfaces: {
                match iterator.peek() {
                    Some(_) => Some(ImplementsInterfaces::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            directives: {
                match iterator.peek() {
                    Some(_) => Some(Directives::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            fields_definition: {
                match iterator.peek() {
                    Some(_) => Some(FieldsDefinition::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InterfaceTypeDefinition {
    description: Option<Description>,
    name: Name,
    implements_interfaces: Option<ImplementsInterfaces>,
    directives: Option<Directives>,
    fields_definition: Option<FieldsDefinition>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for InterfaceTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: {
                let next = iterator.peek().unwrap();
                match next.as_rule() {
                    Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                    _ => None,
                }
            },
            name: Name::try_from(iterator.next().unwrap())?,
            implements_interfaces: {
                match iterator.peek() {
                    Some(_) => Some(ImplementsInterfaces::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            directives: {
                match iterator.peek() {
                    Some(_) => Some(Directives::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            fields_definition: {
                match iterator.peek() {
                    Some(_) => Some(FieldsDefinition::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UnionTypeDefinition {
    description: Option<Description>,
    name: Name,
    directives: Option<Directives>,
    union_member_types: Option<UnionMemberTypes>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for UnionTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: {
                match iterator.peek() {
                    Some(_) => Some(Description::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            name: Name::try_from(iterator.next().unwrap())?,
            directives: match iterator.peek() {
                Some(_) => Some(Directives::try_from(iterator.next().unwrap())?),
                None => None,
            },
            union_member_types: match iterator.peek() {
                Some(_) => Some(UnionMemberTypes::try_from(iterator.next().unwrap())?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UnionMemberTypes(pub Vec<NamedType>);

impl<'a> TryFrom<Pair<'a, Rule>> for UnionMemberTypes {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut output = vec![];
        for token in iterator {
            output.push(NamedType::try_from(token)?);
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EnumTypeDefinition {
    description: Option<Description>,
    name: Name,
    directives: Option<Directives>,
    enum_values_definition: Option<EnumValuesDefinition>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for EnumTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: match iterator.peek().unwrap().as_rule() {
                Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            name: Name::try_from(iterator.next().unwrap())?,
            directives: {
                match iterator.peek() {
                    Some(_) => Some(Directives::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            enum_values_definition: match iterator.peek() {
                Some(_) => Some(EnumValuesDefinition::try_from(iterator.next().unwrap())?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EnumValuesDefinition(pub Vec<EnumValueDefinition>);

impl<'a> TryFrom<Pair<'a, Rule>> for EnumValuesDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut output = vec![];
        for token in iterator {
            output.push(EnumValueDefinition::try_from(token)?);
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EnumValueDefinition {
    description: Option<Description>,
    enum_value: EnumValue,
    directives: Option<Directives>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for EnumValueDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: {
                match iterator.peek() {
                    Some(_) => Some(Description::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            enum_value: EnumValue::try_from(iterator.next().unwrap())?,
            directives: {
                match iterator.peek() {
                    Some(_) => Some(Directives::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct EnumValue(pub Name);

impl<'a> TryFrom<Pair<'a, Rule>> for EnumValue {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(Name::try_from(pair.into_inner().next().unwrap())?))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputObjectTypeDefinition {
    description: Option<Description>,
    name: Name,
    directives: Option<Directives>,
    input_fields_definition: Option<InputFieldsDefinition>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for InputObjectTypeDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: {
                match iterator.peek().unwrap().as_rule() {
                    Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                    _ => None,
                }
            },
            name: { Name::try_from(iterator.next().unwrap())? },
            directives: {
                match iterator.peek() {
                    Some(item) => match item.as_rule() {
                        Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
                        _ => None,
                    },
                    None => None,
                }
            },
            input_fields_definition: match iterator.peek() {
                Some(item) => match item.as_rule() {
                    Rule::input_fields_definition => {
                        Some(InputFieldsDefinition::try_from(iterator.next().unwrap())?)
                    }
                    _ => None,
                },
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputFieldsDefinition(pub Vec<InputValueDefinition>);

impl<'a> TryFrom<Pair<'a, Rule>> for InputFieldsDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut output = vec![];
        for token in iterator {
            output.push(InputValueDefinition::try_from(token)?);
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputValueDefinition {
    description: Option<Description>,
    name: Name,
    graphql_type: GraphQLType,
    default_value: Option<DefaultValue>,
    directives: Option<Directives>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for InputValueDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            description: {
                match iterator.peek().unwrap().as_rule() {
                    Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                    _ => None,
                }
            },
            name: Name::try_from(iterator.next().unwrap())?,
            graphql_type: { GraphQLType::try_from(iterator.next().unwrap())? },
            default_value: {
                match iterator.peek() {
                    Some(_) => Some(DefaultValue::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
            directives: {
                match iterator.peek() {
                    Some(_) => Some(Directives::try_from(iterator.next().unwrap())?),
                    None => None,
                }
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct DefaultValue(pub Value);

impl<'a> TryFrom<Pair<'a, Rule>> for DefaultValue {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(Value::try_from(pair.into_inner().next().unwrap())?))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ListValue(Vec<Value>);

impl<'a> TryFrom<Pair<'a, Rule>> for ListValue {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let list = pair.into_inner();
        let mut output = vec![];
        for item in list {
            output.push(Value::try_from(item)?);
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ObjectValue(pub ObjectField);

impl<'a> TryFrom<Pair<'a, Rule>> for ObjectValue {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(ObjectField::try_from(
            pair.into_inner().next().unwrap(),
        )?))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ObjectField {
    name: Name,
    value: Value,
}

impl<'a> TryFrom<Pair<'a, Rule>> for ObjectField {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut values = pair.into_inner();
        return Ok(Self {
            name: Name::try_from(values.next().unwrap())?,
            value: Value::try_from(values.next().unwrap())?,
        });
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Variable(pub Name);

impl<'a> TryFrom<Pair<'a, Rule>> for Variable {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(Name::try_from(pair.into_inner().next().unwrap())?))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum Value {
    Variable(Variable),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(Name),
    List(ListValue),
    Object(Box<ObjectValue>),
}

impl<'a> TryFrom<Pair<'a, Rule>> for Value {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::variable => Ok(Self::Variable(Variable::try_from(
                pair.into_inner().next().unwrap(),
            )?)),
            Rule::int_value => Ok(Self::Int(match pair.as_str().parse::<i64>() {
                Ok(i) => Ok(i),
                Err(_) => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Couldn't parse {} as an integer.", pair.as_str()),
                    },
                    pair.as_span(),
                )),
            }?)),
            // ^^ sorry for the mess
            Rule::string_value => Ok(Self::String(pair.as_str().to_string())),
            Rule::boolean_value => Ok(Self::Boolean(match pair.as_str().parse::<bool>() {
                Ok(b) => Ok(b),
                Err(_) => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Couldn't parse {} as a boolean.", pair.as_str()),
                    },
                    pair.as_span(),
                )),
            }?)),
            // ^^ again, sorry for the mess
            Rule::null_value => Ok(Self::Null),
            Rule::enum_value => Ok(Self::Enum(Name::try_from(pair)?)),
            Rule::list_value => Ok(Self::List(ListValue::try_from(pair)?)),
            Rule::object_value => Ok(Self::Object(Box::new(ObjectValue::try_from(pair)?))),
            Rule::float_value => Ok(Self::Float(match pair.as_str().parse::<f64>() {
                Ok(i) => Ok(i),
                Err(_) => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Couldn't parse {} as a float.", pair.as_str()),
                    },
                    pair.as_span(),
                )),
            }?)),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum UnionTypeExtension {
    WithMemberTypes {
        name: Name,
        directives: Option<Directives>,
        member_types: UnionMemberTypes,
    },
    WithoutMemberTypes {
        name: Name,
        directives: Directives,
    },
}

impl<'a> TryFrom<Pair<'a, Rule>> for UnionTypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        // ignore "extends union"
        iterator.next();
        let name = Name::try_from(iterator.next().unwrap())?;
        let optional_directives = iterator.peek().unwrap();
        let directives = match optional_directives.as_rule() {
            Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
            _ => None,
        };
        let member_types = if iterator.peek().is_some() {
            Some(UnionMemberTypes::try_from(iterator.next().unwrap())?)
        } else {
            None
        };
        Ok(if let Some(member_types) = member_types {
            Self::WithMemberTypes {
                name,
                directives,
                member_types,
            }
        } else {
            Self::WithoutMemberTypes {
                name,
                directives: directives.unwrap(),
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum InterfaceTypeExtension {
    WithImplementedInterfaces {
        name: Name,
        implements_interfaces: ImplementsInterfaces,
    },
    WithDefinedFields {
        name: Name,
        implements_interfaces: Option<ImplementsInterfaces>,
        directives: Option<Directives>,
        fields_definition: FieldsDefinition,
    },
    WithDirectives {
        name: Name,
        implements_interfaces: Option<ImplementsInterfaces>,
        directives: Directives,
    },
}

impl<'a> TryFrom<Pair<'a, Rule>> for InterfaceTypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        let name = Name::try_from(iterator.next().unwrap())?;
        let implements_interfaces = match iterator.peek().unwrap().as_rule() {
            Rule::implements_interfaces => {
                Some(ImplementsInterfaces::try_from(iterator.next().unwrap())?)
            }
            Rule::directives | Rule::fields_definition => None,
            _ => unreachable!(),
        };
        if let Some(ref implements_interfaces) = implements_interfaces {
            if iterator.peek().is_none() {
                return Ok(Self::WithImplementedInterfaces {
                    name,
                    implements_interfaces: implements_interfaces.clone(),
                });
            }
        }
        let directives = match iterator.peek().unwrap().as_rule() {
            Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
            Rule::fields_definition => None,
            _ => unreachable!(),
        };
        if let Some(ref directives) = directives {
            if iterator.peek().is_none() {
                return Ok(Self::WithDirectives {
                    name,
                    implements_interfaces,
                    directives: directives.clone(),
                });
            }
        }

        Ok(Self::WithDefinedFields {
            name,
            implements_interfaces,
            directives,
            fields_definition: FieldsDefinition::try_from(iterator.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum ObjectTypeExtension {
    WithFields {
        name: Name,
        implements_interfaces: Option<ImplementsInterfaces>,
        directives: Option<Directives>,
        fields_definition: FieldsDefinition,
    },
    WithDirectives {
        name: Name,
        implements_interfaces: Option<ImplementsInterfaces>,
        directives: Directives,
    },
    WithImplementsInterfaces {
        name: Name,
        implements_interfaces: ImplementsInterfaces,
    },
}

impl<'a> TryFrom<Pair<'a, Rule>> for ObjectTypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        let name = Name::try_from(iterator.next().unwrap())?;
        let implements_interfaces = match iterator.peek().unwrap().as_rule() {
            Rule::implements_interfaces => {
                Some(ImplementsInterfaces::try_from(iterator.next().unwrap())?)
            }
            Rule::directives | Rule::fields_definition => None,
            _ => unreachable!(),
        };
        if iterator.peek().is_none() {
            if let Some(implements_interfaces) = implements_interfaces {
                return Ok(Self::WithImplementsInterfaces {
                    name,
                    implements_interfaces,
                });
            }
        }
        let directives = match iterator.peek().unwrap().as_rule() {
            Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
            Rule::fields_definition => None,
            _ => unreachable!(),
        };
        if iterator.peek().is_none() {
            if let Some(directives) = directives {
                return Ok(Self::WithDirectives {
                    name,
                    implements_interfaces,
                    directives,
                });
            }
        }
        let fields_definition = FieldsDefinition::try_from(iterator.next().unwrap())?;
        return Ok(Self::WithFields {
            name,
            implements_interfaces,
            directives,
            fields_definition,
        });
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum EnumTypeExtension {
    WithDirectives {
        name: Name,
        directives: Directives,
    },
    WithEnumValuesDefinition {
        name: Name,
        directives: Option<Directives>,
        enum_values_definition: EnumValuesDefinition,
    },
}

impl<'a> TryFrom<Pair<'a, Rule>> for EnumTypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        iterator.next();
        let name = Name::try_from(iterator.next().unwrap())?;
        let directives = match iterator.peek().unwrap().as_rule() {
            Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
            Rule::enum_values_definition => None,
            _ => unreachable!(),
        };
        if iterator.peek().is_none() {
            if let Some(directives) = directives {
                return Ok(Self::WithDirectives { name, directives });
            }
        }
        return Ok(Self::WithEnumValuesDefinition {
            name,
            directives,
            enum_values_definition: EnumValuesDefinition::try_from(iterator.next().unwrap())?,
        });
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum InputObjectTypeExtension {
    WithDirectives {
        name: Name,
        directives: Directives,
    },
    WithInputFields {
        name: Name,
        directives: Option<Directives>,
        input_fields_definition: InputFieldsDefinition,
    },
}

impl<'a> TryFrom<Pair<'a, Rule>> for InputObjectTypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        iterator.next().unwrap();
        let name = Name::try_from(iterator.next().unwrap())?;
        let directives = match iterator.peek().unwrap().as_rule() {
            Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
            Rule::input_fields_definition => None,
            _ => unreachable!(),
        };
        if iterator.peek().is_none() {
            if let Some(directives) = directives {
                return Ok(Self::WithDirectives { name, directives });
            }
        };
        let input_fields_definition = InputFieldsDefinition::try_from(iterator.next().unwrap())?;
        Ok(Self::WithInputFields {
            name,
            directives,
            input_fields_definition,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum TypeExtension {
    ScalarTypeExtension(ScalarTypeExtension),
    ObjectTypeExtension(ObjectTypeExtension),
    InterfaceTypeExtension(InterfaceTypeExtension),
    UnionTypeExtension(UnionTypeExtension),
    EnumTypeExtension(EnumTypeExtension),
    InputObjectTypeExtension(InputObjectTypeExtension),
}

impl<'a> TryFrom<Pair<'a, Rule>> for TypeExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::scalar_type_extension => Ok(Self::ScalarTypeExtension(
                ScalarTypeExtension::try_from(pair)?,
            )),
            Rule::object_type_extension => Ok(Self::ObjectTypeExtension(
                ObjectTypeExtension::try_from(pair)?,
            )),
            Rule::interface_type_extension => Ok(Self::InterfaceTypeExtension(
                InterfaceTypeExtension::try_from(pair)?,
            )),
            Rule::union_type_extension => Ok(Self::UnionTypeExtension(
                UnionTypeExtension::try_from(pair)?,
            )),
            Rule::enum_type_extension => {
                Ok(Self::EnumTypeExtension(EnumTypeExtension::try_from(pair)?))
            }
            Rule::input_object_type_extension => Ok(Self::InputObjectTypeExtension(
                InputObjectTypeExtension::try_from(pair)?,
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct DirectiveDefinition {
    description: Option<Description>,
    name: Name,
    arguments_definition: Option<ArgumentsDefinition>,
    repeatable: bool,
    directive_locations: DirectiveLocations,
}

impl<'a> TryFrom<Pair<'a, Rule>> for DirectiveDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        println!("{:?}", iterator);
        Ok(Self {
            description: {
                match iterator.peek().unwrap().as_rule() {
                    Rule::description => Some(Description::try_from(iterator.next().unwrap())?),
                    _ => None,
                }
            },
            name: { Name::try_from(iterator.next().unwrap())? },
            arguments_definition: {
                match iterator.peek().unwrap().as_rule() {
                    Rule::arguments_definition => {
                        Some(ArgumentsDefinition::try_from(iterator.next().unwrap())?)
                    }
                    _ => None,
                }
            },
            repeatable: {
                match iterator.peek().unwrap().as_rule() {
                    Rule::repeatable => {
                        iterator.next().unwrap();
                        true
                    }
                    _ => false,
                }
            },
            directive_locations: {
                let x = iterator.next().unwrap();
                DirectiveLocations::try_from(x)?
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct DirectiveLocations(pub Vec<DirectiveLocation>);

impl<'a> TryFrom<Pair<'a, Rule>> for DirectiveLocations {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut result = vec![];
        for item in pair.into_inner() {
            result.push(DirectiveLocation::try_from(item)?);
        }
        Ok(Self(result))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum DirectiveLocation {
    ExecutableDirectiveLocation(ExecutableDirectiveLocation),
    TypeSystemDirectiveLocation(TypeSystemDirectiveLocation),
}

impl<'a> TryFrom<Pair<'a, Rule>> for DirectiveLocation {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        let item = iterator.next().unwrap();
        match item.as_rule() {
            Rule::executable_directive_location => Ok(Self::ExecutableDirectiveLocation(
                ExecutableDirectiveLocation::try_from(item)?,
            )),
            Rule::type_system_directive_location => Ok(Self::TypeSystemDirectiveLocation(
                TypeSystemDirectiveLocation::try_from(item)?,
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum ExecutableDirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
    VariableDefinition,
}

impl<'a> TryFrom<Pair<'a, Rule>> for ExecutableDirectiveLocation {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_str() {
            "QUERY" => Ok(Self::Query),
            "MUTATION" => Ok(Self::Mutation),
            "FIELD" => Ok(Self::Field),
            "SUBSCRIPTION" => Ok(Self::Subscription),
            "FRAGMENT_DEFINITION" => Ok(Self::FragmentDefinition),
            "FRAGMENT_SPREAD" => Ok(Self::FragmentSpread),
            "INLINE_FRAGMENT" => Ok(Self::InlineFragment),
            "VARIABLE_DEFINITION" => Ok(Self::VariableDefinition),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum TypeSystemDirectiveLocation {
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDescription,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputValue,
    InputObject,
    InputFieldDefinition,
}

impl<'a> TryFrom<Pair<'a, Rule>> for TypeSystemDirectiveLocation {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_str() {
            "SCHEMA" => Ok(Self::Schema),
            "SCALAR" => Ok(Self::Scalar),
            "OBJECT" => Ok(Self::Object),
            "FIELD_DEFINITION" => Ok(Self::FieldDefinition),
            "ARGUMENT_DESCRIPTION" => Ok(Self::ArgumentDescription),
            "INTERFACE" => Ok(Self::Interface),
            "UNION" => Ok(Self::Union),
            "ENUM" => Ok(Self::Enum),
            "ENUM_VALUE" => Ok(Self::EnumValue),
            "INPUT_VALUE" => Ok(Self::InputValue),
            "INPUT_OBJECT" => Ok(Self::InputObject),
            "INPUT_FIELD_DEFINITION" => Ok(Self::InputFieldDefinition),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct VariableDefinition {
    variable: Variable,
    graphql_type: GraphQLType,
    default_value: Option<DefaultValue>,
    directives: Option<Directives>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for VariableDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            variable: Variable::try_from(iterator.next().unwrap())?,
            graphql_type: GraphQLType::try_from(iterator.next().unwrap())?,
            default_value: match iterator.peek() {
                Some(item) => match item.as_rule() {
                    Rule::default_value => Some(DefaultValue::try_from(iterator.next().unwrap())?),
                    _ => unreachable!(),
                },
                None => None,
            },
            directives: match iterator.peek() {
                Some(item) => match item.as_rule() {
                    Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
                    _ => unreachable!(),
                },
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct VariableDefinitions(pub Vec<VariableDefinition>);

impl<'a> TryFrom<Pair<'a, Rule>> for VariableDefinitions {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut variable_definitions = vec![];
        for item in iterator {
            variable_definitions.push(VariableDefinition::try_from(item)?)
        }
        Ok(Self(variable_definitions))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
/// A GraphQL operation.
pub struct OperationDefinition {
    pub operation_type: OperationType,
    pub name: Option<Name>,
    pub variable_definitions: Option<VariableDefinitions>,
    pub directives: Option<Directives>,
    pub selection_set: SelectionSet,
}

impl<'a> TryFrom<Pair<'a, Rule>> for OperationDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            operation_type: OperationType::try_from(iterator.next().unwrap())?,
            name: match iterator.peek().unwrap().as_rule() {
                Rule::name => Some(Name::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            variable_definitions: None,
            directives: match iterator.peek().unwrap().as_rule() {
                Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            selection_set: SelectionSet::try_from(iterator.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct FragmentName {
    pub name: Name,
}

impl<'a> TryFrom<Pair<'a, Rule>> for FragmentName {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: Name::try_from(pair.into_inner().next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TypeCondition {
    named_type: NamedType,
}

impl<'a> TryFrom<Pair<'a, Rule>> for TypeCondition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            named_type: NamedType::try_from(iterator.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct FragmentSpread {
    fragment_name: FragmentName,
    directives: Option<Directives>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for FragmentSpread {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            fragment_name: FragmentName::try_from(iterator.next().unwrap())?,
            directives: match iterator.next() {
                Some(t) => Some(Directives::try_from(t)?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InlineFragment {
    type_condition: Option<TypeCondition>,
    directives: Option<Directives>,
    selection_set: SelectionSet,
}

impl<'a> TryFrom<Pair<'a, Rule>> for InlineFragment {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            type_condition: match iterator.peek().unwrap().as_rule() {
                Rule::type_condition => Some(TypeCondition::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            directives: match iterator.peek().unwrap().as_rule() {
                Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            selection_set: SelectionSet::try_from(iterator.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
}

impl<'a> TryFrom<Pair<'a, Rule>> for Selection {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        let item = iterator.next().unwrap();
        match item.as_rule() {
            Rule::field => Ok(Self::Field(Field::try_from(item)?)),
            Rule::fragment_spread => Ok(Self::FragmentSpread(FragmentSpread::try_from(item)?)),
            Rule::inline_fragment => Ok(Self::InlineFragment(InlineFragment::try_from(item)?)),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SelectionSet(Vec<Selection>);

impl<'a> TryFrom<Pair<'a, Rule>> for SelectionSet {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        // ignore opening bracket
        iterator.next().unwrap();
        let mut output = vec![];
        for item in iterator {
            match item.as_rule() {
                Rule::selection => output.push(Selection::try_from(item)?),
                _ => return Ok(Self(output)),
            }
        }
        Ok(Self(output))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
/// A GraphQL fragment.
pub struct FragmentDefinition {
    fragment_name: FragmentName,
    type_condition: TypeCondition,
    directives: Option<Directives>,
    selection_set: SelectionSet,
}

impl<'a> TryFrom<Pair<'a, Rule>> for FragmentDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(Self {
            fragment_name: FragmentName::try_from(iterator.next().unwrap())?,
            type_condition: TypeCondition::try_from(iterator.next().unwrap())?,
            directives: match iterator.peek().unwrap().as_rule() {
                Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
                _ => None,
            },
            selection_set: SelectionSet::try_from(iterator.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum ExecutableDefinition {
    OperationDefinition(OperationDefinition),
    FragmentDefinition(FragmentDefinition),
}

impl<'a> TryFrom<Pair<'a, Rule>> for ExecutableDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        match iterator.peek().unwrap().as_rule() {
            Rule::operation_definition => Ok(Self::OperationDefinition(
                OperationDefinition::try_from(iterator.next().unwrap())?,
            )),
            Rule::fragment_definition => Ok(Self::FragmentDefinition(
                FragmentDefinition::try_from(iterator.next().unwrap())?,
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum TypeSystemDefinition {
    SchemaDefinition(SchemaDefinition),
    TypeDefinition(TypeDefinition),
    DirectiveDefinition(DirectiveDefinition),
}

impl<'a> TryFrom<Pair<'a, Rule>> for TypeSystemDefinition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        match iterator.peek().unwrap().as_rule() {
            Rule::schema_definition => Ok(Self::SchemaDefinition(SchemaDefinition::try_from(
                iterator.next().unwrap(),
            )?)),
            Rule::type_definition => Ok(Self::TypeDefinition(TypeDefinition::try_from(
                iterator.next().unwrap(),
            )?)),
            Rule::directive_definition => Ok(Self::DirectiveDefinition(
                DirectiveDefinition::try_from(iterator.next().unwrap())?,
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum SchemaExtension {
    WithDirectives {
        directives: Directives,
    },
    WithRootOperationTypeDefinition {
        directives: Option<Directives>,
        root_operation_type_definition: RootOperationTypeDefinition,
    },
}

impl<'a> TryFrom<Pair<'a, Rule>> for SchemaExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        let directives = match iterator.peek().unwrap().as_rule() {
            Rule::directives => Some(Directives::try_from(iterator.next().unwrap())?),
            Rule::root_operation_type_definition => None,
            _ => unreachable!(),
        };
        if iterator.peek().is_none() {
            if let Some(directives) = directives {
                return Ok(Self::WithDirectives { directives });
            }
        }
        let root_operation_type_definition =
            RootOperationTypeDefinition::try_from(iterator.next().unwrap())?;
        Ok(Self::WithRootOperationTypeDefinition {
            directives,
            root_operation_type_definition,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum TypeSystemExtension {
    SchemaExtension(SchemaExtension),
    TypeExtension(TypeExtension),
}

impl<'a> TryFrom<Pair<'a, Rule>> for TypeSystemExtension {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(match iterator.peek().unwrap().as_rule() {
            Rule::schema_extension => {
                Self::SchemaExtension(SchemaExtension::try_from(iterator.next().unwrap())?)
            }
            Rule::type_extension => {
                Self::TypeExtension(TypeExtension::try_from(iterator.next().unwrap())?)
            }
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum Definition {
    ExecutableDefinition(ExecutableDefinition),
    TypeSystemDefinition(TypeSystemDefinition),
    TypeSystemExtension(TypeSystemExtension),
}

impl<'a> TryFrom<Pair<'a, Rule>> for Definition {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut iterator = pair.into_inner();
        Ok(match iterator.peek().unwrap().as_rule() {
            Rule::executable_definition => Self::ExecutableDefinition(
                ExecutableDefinition::try_from(iterator.next().unwrap())?,
            ),
            Rule::type_system_definition => Self::TypeSystemDefinition(
                TypeSystemDefinition::try_from(iterator.next().unwrap())?,
            ),
            Rule::type_system_extension => {
                Self::TypeSystemExtension(TypeSystemExtension::try_from(iterator.next().unwrap())?)
            }
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Document(Vec<Definition>);

impl<'a> TryFrom<Pair<'a, Rule>> for Document {
    type Error = Error<Rule>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let iterator = pair.into_inner();
        let mut definitions = vec![];
        for item in iterator {
            definitions.push(Definition::try_from(item)?);
        }
        Ok(Self(definitions))
    }
}
