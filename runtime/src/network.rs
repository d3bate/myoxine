use serde::Deserialize;

use crate::query::Query;

use http::Request;
use yew::Callback;

/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

thread_local! {
    /// Stores the currently ongoing HTTP requests.
    static REQUESTS: OngoingRequests = OngoingRequests::default();
}

/// Stores the details of ongoing requests.
///
/// This struct is internal to the library and should not be used externally.
#[derive(Default)]
struct OngoingRequests {}

/// A network connection. This is a modular piece which can be swapped in and out to suit your needs
/// (without paying substantial runtime cost). The trait does, however, require that you use the
/// types from the `http` crate (which is a pretty common crate).
///
/// The original reason this was made into a trait was to allow for more complex network setups,
/// particularly around p2p connections in a web browser. This allows you to construct an interface
/// where it isn't important how data is fetched, so long as it is fetched.
pub trait Network {
    fn dispatch<OUT>(request: Request<String>, callback: Callback<OUT>);
}

pub struct VanillaNetwork {}

impl Network for VanillaNetwork {
    fn dispatch<OUT>(request: Request<String>, callback: Callback<OUT>) {
        todo!()
    }
}

#[cfg(test)]
mod test_vanilla_network {
    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }
}
