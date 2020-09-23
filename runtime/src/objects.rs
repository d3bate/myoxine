use crate::query::Query;

/// A trait which should be implemented on any type representing a GraphQL object. This trait is not
/// intended for manual implementation; you should instead use our derive macro.
pub trait Object {
    /// This function refetches an object from the GraphQL server.
    ///
    /// Note that Myoxine makes some assumptions about what you have named the refetch query. If you
    /// have named your query to something other than the name the derive macro automatically
    /// infers you will need to specify the `#[refetch_query = "query"]` attribute.
    fn refetch_object(&self) -> Query;
}
