+++
title="#[derive(Query)]"
+++
The query macro generates the code needed to execute a GraphQL query. It parses a GraphQL query
supplied using the `#[query="<some query>"]` attribute. This query is then type-checked against the
schema. If type checking is successful, the macro then implements the `Query` trait on the type.
This enables you to use it with the Myoxine runtime.