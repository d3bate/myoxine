/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/

//! Custom tests for the ast. Tests to prevent regressions for bugs in the ast should not be
//! included in this file!

use ast::parse_string;

fn assert_parses(input: &str) {
    let parsed = parse_string(input);
    assert!(parsed.is_ok());
}

#[test]
fn test_directives_cache_control() {
    assert_parses(
        r#"type Post @cacheControl(maxAge: 240) {
        id: Int!
        title: String
        author: Author
        votes: Int @cacheControl(maxAge: 30)
        comments: [Comment]
        readByCurrentUser: Boolean! @cacheControl(scope: PRIVATE)
      }
      
      type Comment @cacheControl(maxAge: 1000) {
        post: Post!
      }
      
      type Query {
        latestPost: Post @cacheControl(maxAge: 14)
      }"#,
    );
}
