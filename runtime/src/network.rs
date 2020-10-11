use serde::Deserialize;

use crate::query::Query;

/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

pub trait NetworkExecutor {
    fn execute_query(&self, input: Query) -> String;
}

pub trait Network: NetworkExecutor {}
