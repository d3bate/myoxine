/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/

//! Tests that all the examples in the GraphQL specification can be parsed. This is done on a
//! best-effort basis and may lag behind the specification.

use parser;

fn assert_parses(input: &str) {
    let parsed = parser::parse_string(input);
    assert!(parsed.is_ok());
}

#[test]
fn test_example_38_type_definition() {
    let schema = r#"type Query {
      myName: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_40_type_definition() {
    let schema = r#"schema {
      query: MyQueryRootType
      mutation: MyMutationRootType
    }

    type MyQueryRootType {
      someField: String
    }

    type MyMutationRootType {
      setSomeField(to: String): String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_41_type_definition() {
    let schema = r#"type Query {
      someField: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_42_scalars() {
    let schema = r#"scalar Time
    scalar Url
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_43_type_person() {
    let schema = r#"type Person {
      name: String
      age: Int
      picture: Url
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_48_relationships() {
    let schema = r#"type Person {
      name: String
      age: Int
      picture: Url
      relationship: Person
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_52_fragment() {
    let schema = r#"fragment Frag on Query {
      bar
      baz
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_54_fragment() {
    let schema = r#"fragment Ignored on UnknownType {
      qux
      baz
    }

    fragment Matching on Query {
      bar
      qux
      foo
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_58_type_person() {
    let schema = r#"type Person {
      name: String
      picture(size: Int): Url
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_61_field_deprecation() {
    let schema = r#"type ExampleType {
      oldField: String @deprecated
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_62_extend_type() {
    let schema = r#"extend type Story {
      isHiddenLocally: Boolean
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_63_extend_with_directives() {
    let schema = r#"extend type User @addedDirective
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_64_interfaces() {
    let schema = r#"interface NamedEntity {
      name: String
    }

    interface ValuedEntity {
      value: Int
    }

    type Person implements NamedEntity {
      name: String
      age: Int
    }

    type Business implements NamedEntity & ValuedEntity {
      name: String
      value: Int
      employeeCount: Int
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_65_interfaces() {
    let schema = r#"type Contact {
      entity: NamedEntity
      phoneNumber: String
      address: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_69_interfaces_implementing_interfaces() {
    let schema = r#"interface Node {
      id: ID!
    }

    interface Resource implements Node {
      id: ID!
      url: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_70_transitive_interfaces() {
    let schema = r#"interface Node {
      id: ID!
    }

    interface Resource implements Node {
      id: ID!
      url: String
    }

    interface Image implements Resource & Node {
      id: ID!
      url: String
      thumbnail: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_71_counter_example() {
    // even though this is semantically invalid
    // it is syntactically valid
    let schema = r#"interface Node implements Named & Node {
      id: ID!
      name: String
    }

    interface Named implements Node & Named {
      id: ID!
      name: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_72_interface_extensions() {
    let schema = r#"extend interface NamedEntity {
      nickname: String
    }

    extend type Person {
      nickname: String
    }

    extend type Business {
      nickname: String
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_73_directives() {
    let schema = r#"extend interface NamedEntity @addedDirective
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_74_unions() {
    let schema = r#"union SearchResult = Photo | Person
    type Person {
      name: String
      age: Int
    }

    type Photo {
      height: Int
      width: Int
    }

    type SearchQuery {
      firstSearchResult: SearchResult
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_77_alternative_syntax() {
    let schema = r#"
    union SearchResult =
      | Photo
      | Person
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_78_direction_enum() {
    let schema = r#"
    enum Direction {
      NORTH
      EAST
      SOUTH
      WEST
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_79_input_objects() {
    let schema = r#"input Point2D {
      x: Float
      y: Float
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_80_input_objects() {
    let schema = r#"input ExampleInputObject {
      a: String
      b: Int!
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_83() {
    let schema = r#"query withNullableVariable($var: String) {
      fieldWithNonNullArg(nonNullArg: $var)
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_84_basic_directive() {
    let schema = r#"directive @example on FIELD

    fragment SomeFragment on SomeType {
      field @example
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_85_directive_optional_syntax() {
    let schema = r#"directive @example on FIELD

    fragment SomeFragment on SomeType {
      field @example
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_86_metadata() {
    let schema = r#"directive @example on FIELD_DEFINITION | ARGUMENT_DEFINITION

    type SomeType {
      field(arg: Int @example): String @example
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_87_repeatable() {
    let schema = r#"directive @delegateField(name: String!) repeatable on OBJECT | INTERFACE

    type Book @delegateField(name: "pageCount") @delegateField(name: "author") {
      id: ID!
    }

    extend type Book @delegateField(name: "index")
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_89_skip_directive() {
    let schema = r#"query myQuery($someTest: Boolean!) {
      experimentalField @skip(if: $someTest)
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_90_include_directive() {
    let schema = r#"query myQuery($someTest: Boolean!) {
      experimentalField @include(if: $someTest)
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_91_deprecated_directive() {
    let schema = r#"type ExampleType {
      newField: String
      oldField: String @deprecated(reason: "Use `newField`.")
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_92_introspection() {
    let schema = r#"
    type User {
      id: String
      name: String
      birthday: Date
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_introspection_system_schema() {
    let schema = r#"
    type __Schema {
      description: String
      types: [__Type!]!
      queryType: __Type!
      mutationType: __Type
      subscriptionType: __Type
      directives: [__Directive!]!
    }

    type __Type {
      kind: __TypeKind!
      name: String
      description: String

      # should be non-null for OBJECT and INTERFACE only, must be null for the others
      fields(includeDeprecated: Boolean = false): [__Field!]

      # should be non-null for OBJECT and INTERFACE only, must be null for the others
      interfaces: [__Type!]

      # should be non-null for INTERFACE and UNION only, always null for the others
      possibleTypes: [__Type!]

      # should be non-null for ENUM only, must be null for the others
      enumValues(includeDeprecated: Boolean = false): [__EnumValue!]

      # should be non-null for INPUT_OBJECT only, must be null for the others
      inputFields: [__InputValue!]

      # should be non-null for NON_NULL and LIST only, must be null for the others
      ofType: __Type
    }

    type __Field {
      name: String!
      description: String
      args: [__InputValue!]!
      type: __Type!
      isDeprecated: Boolean!
      deprecationReason: String
    }

    type __InputValue {
      name: String!
      description: String
      type: __Type!
      defaultValue: String
    }

    type __EnumValue {
      name: String!
      description: String
      isDeprecated: Boolean!
      deprecationReason: String
    }

    enum __TypeKind {
      SCALAR
      OBJECT
      INTERFACE
      UNION
      ENUM
      INPUT_OBJECT
      LIST
      NON_NULL
    }

    type __Directive {
      name: String!
      description: String
      locations: [__DirectiveLocation!]!
      args: [__InputValue!]!
      isRepeatable: Boolean!
    }

    enum __DirectiveLocation {
      QUERY
      MUTATION
      SUBSCRIPTION
      FIELD
      FRAGMENT_DEFINITION
      FRAGMENT_SPREAD
      INLINE_FRAGMENT
      SCHEMA
      SCALAR
      OBJECT
      FIELD_DEFINITION
      ARGUMENT_DEFINITION
      INTERFACE
      UNION
      ENUM
      ENUM_VALUE
      INPUT_OBJECT
      INPUT_FIELD_DEFINITION
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_95_input_object() {
    let schema = r#"
    input Point {
      x: Int
      y: Int
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_96_demo_schema() {
    let schema = r#"
    type Query {
      dog: Dog
    }

    enum DogCommand { SIT, DOWN, HEEL }

    type Dog implements Pet {
      name: String!
      nickname: String
      barkVolume: Int
      doesKnowCommand(dogCommand: DogCommand!): Boolean!
      isHousetrained(atOtherHomes: Boolean): Boolean!
      owner: Human
    }

    interface Sentient {
      name: String!
    }

    interface Pet {
      name: String!
    }

    type Alien implements Sentient {
      name: String!
      homePlanet: String
    }

    type Human implements Sentient {
      name: String!
      pets: [Pet!]
    }

    enum CatCommand { JUMP }

    type Cat implements Pet {
      name: String!
      nickname: String
      doesKnowCommand(catCommand: CatCommand!): Boolean!
      meowVolume: Int
    }

    union CatOrDog = Cat | Dog
    union DogOrHuman = Dog | Human
    union HumanOrAlien = Human | Alien
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_104_fragment_with_subscription() {
    let schema = r#"subscription sub {
      ...newMessageFields
    }

    fragment newMessageFields on Subscription {
      newMessage {
        body
        sender
      }
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_109_interface_fragment() {
    let schema = r#"fragment interfaceFieldSelection on Pet {
      name
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_111_more_complex_fragment() {
    let schema = r#"fragment inDirectFieldSelectionOnUnion on CatOrDog {
      __typename
      ... on Pet {
        name
      }
      ... on Dog {
        barkVolume
      }
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_113_merge_fragments() {
    let schema = r#"fragment mergeIdenticalFields on Dog {
      name
      name
    }

    fragment mergeIdenticalAliasesAndFields on Dog {
      otherName: name
      otherName: name
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_115_merge_identical() {
    let schema = r#"fragment mergeIdenticalFieldsWithIdenticalArgs on Dog {
      doesKnowCommand(dogCommand: SIT)
      doesKnowCommand(dogCommand: SIT)
    }

    fragment mergeIdenticalFieldsWithIdenticalValues on Dog {
      doesKnowCommand(dogCommand: $dogCommand)
      doesKnowCommand(dogCommand: $dogCommand)
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_121_query_extension() {
    let schema = r#"extend type Query {
      human: Human
      pet: Pet
      catOrDog: CatOrDog
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_123_argument_names() {
    let schema = r#"fragment argOnRequiredArg on Dog {
      doesKnowCommand(dogCommand: SIT)
    }

    fragment argOnOptional on Dog {
      isHousetrained(atOtherHomes: true) @include(if: true)
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_126_complicated_arguments() {
    let schema = r#"type Arguments {
      multipleReqs(x: Int!, y: Int!): Int!
      booleanArgField(booleanArg: Boolean): Boolean
      floatArgField(floatArg: Float): Float
      intArgField(intArg: Int): Int
      nonNullBooleanArgField(nonNullBooleanArg: Boolean!): Boolean!
      booleanListArgField(booleanListArg: [Boolean]!): [Boolean]
      optionalNonNullBooleanArgField(optionalBooleanArg: Boolean! = false): Boolean!
    }

    extend type Query {
      arguments: Arguments
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_127_fragments() {
    let schema = r#"fragment multipleArgs on Arguments {
      multipleReqs(x: 1, y: 2)
    }

    fragment multipleArgsReverseOrder on Arguments {
      multipleReqs(y: 2, x: 1)
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_132() {
    // TODO: add support for the query shorthand
    let schema = r#"query {
      dog {
        ...fragmentOne
        ...fragmentTwo
      }
    }

    fragment fragmentOne on Dog {
      name
    }

    fragment fragmentTwo on Dog {
      owner {
        name
      }
    }
    "#;
    assert_parses(schema);
}

#[test]
fn test_example_134() {
    let schema = "fragment correctType on Dog {
      name
    }

    fragment inlineFragment on Dog {
      ... on Dog {
        name
      }
    }

    fragment inlineFragment2 on Dog {
      ... @include(if: true) {
        name
      }
    }
    ";
    assert_parses(schema);
}
