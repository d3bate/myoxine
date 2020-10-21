use std::marker::PhantomData;

use serde::Deserialize;

/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

/// A query containing a string containing the query to be dispatched to the server.
pub struct Query<OUT>(pub String, PhantomData<OUT>)
where
    OUT: for<'de> Deserialize<'de>;

impl<OUT> Query<OUT>
where
    OUT: for<'de> Deserialize<'de>,
{
    /// Constructs a new query.
    pub fn new(query: String) -> Self {
        Self(query, PhantomData)
    }
    /// Deserialized a JSON stream from the server into the output type of the query.
    pub fn deserialize(result: String) -> Result<OUT, serde_json::Error> {
        serde_json::from_str(&result)
    }
}

impl<OUT> ToString for Query<OUT>
where
    OUT: for<'de> Deserialize<'de>,
{
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
