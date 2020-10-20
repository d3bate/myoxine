use std::{fmt::Write, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::query::Query;

pub struct Unit {}

impl Write for Unit {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        Ok(())
    }
}

/// A trait which should be implemented on any type representing a GraphQL object. This trait is not
/// intended for manual implementation; you should instead use our derive macro.
pub trait Object: for<'de> Deserialize<'de> + Serialize + 'static {
    type Id;
    /// This function returns the id of an object. In most cases this will just return the field on
    /// the object used to represent your GraphQL type as a Rust object.
    fn id(&self) -> &Self::Id;
    /// This function refetches an object from the GraphQL server.
    ///
    /// Note that Myoxine makes some assumptions about what you have named the refetch query. If you
    /// have named your query to something other than the name the derive macro automatically
    /// infers you will need to specify the `#[refetch_query = "query"]` attribute.
    fn refetch_object(&self) -> Query;
}
