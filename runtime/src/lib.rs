/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Mozilla Public License (2.0).
A copy of the license can be found at the root of this Git repository.
*/
//! Myoxine is a GraphQL runtime for Rust. It's designed with Yew and WebAssembly in mind and is
//! intended to make complex applications easy to build and scale. It's currently pretty
//! experimental stuff, but if you like living at the blazing edge of exciting web design software
//! is definitely worth a try. Myoxine is inspired by Javascript GraphQL runtimes such as Relay and
//! Apollo GraphQL.

use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use unsafe_any::UnsafeAny;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub enum Event {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct CacheCallback {
    fires_on: Event,
    callback: Callback<Box<dyn Any>>,
}

pub struct Cache {
    map: anymap::Map,
    callbacks: HashMap<TypeId, CacheCallback>,
}

impl Cache {
    /// Calls every
    pub fn insert<Q>(&mut self, item: Q)
    where
        Q: Query + 'static + Clone,
    {
        self.map.insert::<Q>(item.clone());
        self.callbacks
            .iter()
            .filter(|item| item.0 == &TypeId::of::<Q>())
            .map(|callback| {
                callback.1.callback.emit(Box::new(item.clone()));
            })
            .for_each(drop);
    }
    pub fn register_callback<Q>(&mut self, callback: Callback<Q>, event: Event)
    where
        Q: Query + 'static,
    {
        self.callbacks.insert(
            TypeId::of::<Q>(),
            CacheCallback {
                fires_on: event,
                callback: callback.reform(|any: Box<dyn Any>| *any.downcast::<Q>().unwrap()),
            },
        );
    }
    pub fn remove<Q>(&mut self)
    where
        Q: Query + 'static,
    {
        self.callbacks.remove(&TypeId::of::<Q>());
        self.map.remove::<Q>();
    }
}

pub trait Query {}

#[derive(Properties, Clone)]
pub struct QueryProviderProps<Q>
where
    Q: Query + Clone,
{
    render: Callback<Q>,
}

pub struct QueryProvider<Q>
where
    Q: Query,
{
    _query: PhantomData<Q>,
}

impl<Q: 'static> Component for QueryProvider<Q>
where
    Q: Query + Clone,
{
    type Message = ();
    type Properties = QueryProviderProps<Q>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        unimplemented!()
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}

pub trait Mutation {}

pub struct MutationProvider<M>
where
    M: Mutation,
{
    _mutation: PhantomData<M>,
}

#[derive(Properties, Clone)]
pub struct MutationProviderProps<M>
where
    M: Mutation + Clone,
{
    render: Callback<M>,
}

impl<M: 'static> Component for MutationProvider<M>
where
    M: Mutation + Clone,
{
    type Message = ();
    type Properties = MutationProviderProps<M>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        unimplemented!()
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        unimplemented!()
    }
}
