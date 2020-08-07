//! Useful macros for creating GraphQL queries inside applications.
mod mutation;
mod query;

extern crate proc_macro;

#[proc_macro]
/// A macro for creating GraphQL queries.
///
/// ```rust
/// use macros::query;
/// let user_id = 1;
/// query! {
///     {
///         getUserById(id: user_id) {
///             id,
///             name,
///             email
///         }
///     }
/// }
/// ```
///
/// The macro will output an item implementing `Query`. Many of the functions and
/// methods Myoxine provides have trait bounds which restrict them to take anything
/// implementing `Query`.
pub fn query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[proc_macro]
/// A macro for generating GraphQL mutations.
/// ```rust
/// use macros::mutation;
/// let user_id = 1;
/// let value = "Some Value".to_string(); // needs to be able to take ownership of this
///                                       // i.e. must be `Clone` or `Copy` or not referenced
///                                       // later in the program.
/// mutation! {
///     updateUserById(id: user_id, value: value) {
///         id,
///         username,
///         email
///     }
/// }
/// ```
pub fn mutation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
