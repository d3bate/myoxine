/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/
//! Myoxine is a GraphQL runtime for Rust. It's designed with Yew and WebAssembly in mind and is
//! intended to make complex applications easy to build and scale. It's currently experimental and
//! hasn't been used in the context of a serious application, but hopefully that will change soon.

pub use yew;

pub mod cache;
pub mod network;
pub mod objects;
pub mod query;
pub mod query_provider;

pub type Id = String;
