use std::fmt::Debug;

use serde::Deserialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::query::Query;

use http::Request;
use js_sys::Array;
use yew::Callback;

use std::cell::RefCell;
use std::iter::FromIterator;
use std::thread::LocalKey;

/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

/// A network connection. This is a modular piece which can be swapped in and out to suit your needs
/// (without paying substantial runtime cost). The trait does, however, require that you use the
/// types from the `http` crate (which is a pretty common crate).
///
/// The reason I made this was made into a trait was to allow for more complex network setups,
/// particularly around p2p connections in a web browser. This allows you to construct an interface
/// where it isn't important how data is fetched, so long as it is fetched.
///
/// Note that this trait is intended to be used as a singleton).
pub trait Network: Sized {
    /// Dispatches a query to the internet. The sentence before is phrased like that because you
    /// don't have to use GraphQL for server-client communication – it's also possible to use it for
    /// communication between clients using WebRTC.
    fn dispatch<OUT>(&mut self, query: Query<OUT>, callback: Callback<OUT>)
    where
        OUT: for<'de> Deserialize<'de> + 'static;
    fn add_connection_customiser(&mut self, connection_customiser: Box<dyn CustomiseConnection>);
    /// Returns a `LocalKey` pointing to an instance of this object. This should be done using a
    /// `LocalKey` (via the `thread_local!` macro).
    fn local_key() -> &'static LocalKey<RefCell<Self>>;
}

pub trait CustomiseConnection: Debug {
    /// Customises an HTTP request. Types implementing this trait can be passed to an implementor
    /// of `Network` which *should* call this function before dispatching the request.
    fn customise(&self, request: &mut Request<String>);
}

#[derive(Debug)]
/// The default network implementation. If you want to make requests to a HTTP server, this is what
/// you're looking for. For more elaborate setups, consider something different.
pub struct VanillaNetwork {
    connection_customiser: Option<Box<dyn CustomiseConnection>>,
}

/// Turns a Rust request from the `http` crate into a JS `Request` type.
fn request2js(request: Request<String>) -> yew::web_sys::Request {
    let new_request = yew::web_sys::Request::new_with_str(&request.uri().to_string()).unwrap();
    yew::web_sys::Request::new_with_request_and_init(
        &new_request,
        yew::web_sys::RequestInit::new()
            .headers(&Array::from_iter(request.headers().iter().map(
                |(name, value)| {
                    Array::from_iter(&[
                        JsValue::from(name.to_string()),
                        JsValue::from(value.to_str().unwrap()),
                    ])
                },
            )))
            .body(Some(&request.body().into())),
    )
    .expect("failed to build request")
}

thread_local! {
    pub static VANILLA_NETWORK: RefCell<VanillaNetwork> = RefCell::new(VanillaNetwork::new());
}

impl VanillaNetwork {
    fn new() -> Self {
        Self {
            connection_customiser: None,
        }
    }
}

impl Network for VanillaNetwork {
    fn dispatch<OUT>(&mut self, query: Query<OUT>, callback: Callback<OUT>)
    where
        OUT: for<'de> Deserialize<'de> + 'static,
    {
        let mut request = Request::builder()
            .body(query.to_string())
            .expect("failed to build request – this is an internal error and should be reported to https://github.com/d3bate/myoxine");
        if let Some(connection_customiser) = &self.connection_customiser {
            connection_customiser.customise(&mut request);
        }
        let request = request2js(request);
        let future = JsFuture::from(yew::utils::window().fetch_with_request(&request));
        wasm_bindgen_futures::spawn_local({
            async move {
                // fear not, proper error management should be imminent
                let result = future.await.expect("failed to complete the request");
                let output = result.into_serde::<OUT>().expect("failed to serialize");
                callback.emit(output);
            }
        });
    }

    fn add_connection_customiser(&mut self, connection_customiser: Box<dyn CustomiseConnection>) {
        self.connection_customiser = Some(connection_customiser);
    }

    fn local_key() -> &'static LocalKey<RefCell<Self>> {
        &VANILLA_NETWORK
    }
}

#[cfg(test)]
mod test_vanilla_network {
    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }
}
