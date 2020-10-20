use super::{
    Definition, Document, GraphQLType, Name, NamedType, SchemaDefinition, TypeSystemDefinition,
};

/// I know it's strange to stick impls in a separate file, but this seemed to be a sensible move
/// here because the file was getting too large to be of use otherwise.
impl Document {
    pub fn check_type_exists(&self, name: &Name) -> bool {
        self.get_type(name).is_some()
    }
    pub fn get_type(&self, name: &Name) -> Option<&Definition> {
        self.0
            .iter()
            .filter(|definition| match definition {
                Definition::TypeSystemDefinition(type_system_definition) => {
                    match type_system_definition {
                        TypeSystemDefinition::TypeDefinition(type_definition) => {
                            &Name::from(type_definition.clone()) == name
                        }
                        _ => false,
                    }
                }
                _ => false,
            })
            .next()
    }
    pub fn get_schema_definition(&self) -> Option<SchemaDefinition> {
        self.0
            .iter()
            .filter_map(|definition| match definition {
                Definition::TypeSystemDefinition(type_system_definition) => {
                    match type_system_definition {
                        TypeSystemDefinition::SchemaDefinition(def) => Some(def),
                        _ => None,
                    }
                }
                _ => None,
            })
            .next()
            .map(Clone::clone)
    }
}

impl GraphQLType {
    /// Allows you to extract the underlying name of a type.
    ///
    /// I fear that the recursive structure of names deviates somewhat from the specification.
    pub fn extract_name(&self) -> &NamedType {
        match self {
            Self::NamedType(nt) => nt,
            Self::ListType(lt) => lt.extract_name(),
            Self::NonNullType(nn) => nn.extract_name(),
        }
    }
}

#[cfg(test)]
mod test_check_type_exists {
    use crate::{ast::Name, parse_string};

    #[test]
    fn check_can_find_types() {
        let examples = [
            (r#"type Post { id: Int! }"#, "Comment", false),
            (r#"type Comment { id: Int! }"#, "Post", false),
        ];
        for (example, to_check, should_exist) in examples.iter() {
            let parsed = parse_string(example).expect("Parse error");
            assert_eq!(
                parsed.check_type_exists(&Name(to_check.to_string())),
                *should_exist
            )
        }
    }
}

#[cfg(test)]
pub mod test_get_type_fields {
    #[test]
    fn check_can_get_object_fields() {
        todo!()
    }
}

#[cfg(test)]
pub mod test_get_schema_definition {
    #[test]
    fn test_can_get_schema_definition() {
        todo!()
    }
}
