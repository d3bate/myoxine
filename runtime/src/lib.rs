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
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use unsafe_any::UnsafeAny;
use yew::prelude::*;

thread_local! {
    pub(crate) static CACHE: RefCell<Cache> = RefCell::new(Cache::default());
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
    callbacks: HashMap<TypeId, Vec<CacheCallback>>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            map: anymap::Map::new(),
            callbacks: HashMap::default(),
        }
    }
}

impl Cache {
    /// Inserts and item into the cache and calls all the subscribing callbacks.
    pub fn insert<Q>(&mut self, item: Q)
    where
        Q: Query + 'static + Clone,
    {
        self.map.insert::<Q>(item.clone());
        self.callbacks
            .iter()
            .filter(|item| item.0 == &TypeId::of::<Q>())
            .map(|callback| {
                callback
                    .1
                    .iter()
                    .map(|callback| callback.callback.emit(Box::new(item.clone())))
                    .for_each(drop);
            })
            .for_each(drop);
    }
    /// Register a new callback.
    pub fn register_callback<Q>(&mut self, callback: Callback<Q>, event: Event)
    where
        Q: Query + 'static,
    {
        if let Some(item) = self
            .callbacks
            .iter_mut()
            .find(|key| key.0 == &TypeId::of::<Q>())
        {
            item.1.push(CacheCallback {
                fires_on: event,
                callback: callback.reform(|item: Box<dyn Any>| *item.downcast::<Q>().unwrap()),
            })
        } else {
            self.callbacks.insert(TypeId::of::<Q>(), {
                let mut vec = Vec::new();
                vec.push(CacheCallback {
                    fires_on: event,
                    callback: callback.reform(|any: Box<dyn Any>| *any.downcast::<Q>().unwrap()),
                });
                vec
            });
        }
    }
    /// Removes an item from the cache.
    pub fn remove<Q>(&mut self)
    where
        Q: Query + 'static,
    {
        self.callbacks.remove(&TypeId::of::<Q>());
        self.map.remove::<Q>();
    }
    pub fn get<Q>(&self) -> Option<&Q>
    where
        Q: Query + 'static,
    {
        self.map.get::<Q>()
    }
    pub fn update<Q>(&mut self, new_value: Q)
    where
        Q: Query + 'static + Clone,
    {
        // todo: look for parent/child query types
        // probably needs some unsafe type dynamic type manipulation
        let mut item = self.map.get_mut::<Q>().unwrap();
        let mut new_value = new_value.clone();
        item = &mut new_value;
        for callback_list in self.callbacks.get(&TypeId::of::<Q>()) {
            for callback in callback_list {
                callback.callback.emit(Box::new(item.clone()));
            }
        }
    }
}

pub trait Query {}

#[derive(Properties, Clone)]
pub struct QueryProviderProps<Q>
where
    Q: Query + Clone,
{
    /// A referenced-counted function which is used to render the results of a query.
    render: std::rc::Rc<dyn Fn(Option<Q>) -> Html>,
}

pub struct QueryProvider<Q>
where
    Q: Query + Clone + 'static,
{
    render: std::rc::Rc<dyn Fn(Option<Q>) -> Html>,
    _link: ComponentLink<Self>,
    _query: PhantomData<Q>,
}

pub enum QueryProviderMsg<Q>
where
    Q: Query + Clone,
{
    Update(Q),
}

impl<Q: 'static> Component for QueryProvider<Q>
where
    Q: Query + Clone,
{
    type Message = QueryProviderMsg<Q>;
    type Properties = QueryProviderProps<Q>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // attach a callback
        CACHE.with(|cache| {
            cache
                .borrow_mut()
                .register_callback(link.callback(|q| Self::Message::Update(q)), Event::Updated)
        });
        Self {
            render: props.render,
            _link: link,
            _query: PhantomData,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            // ignore the actual data, just rerender
            Self::Message::Update(_) => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        // the `render` function can be updated here
        if Rc::ptr_eq(&props.render, &self.render) {
            false
        } else {
            self.render = props.render;
            true
        }
    }

    fn view(&self) -> Html {
        self.render.deref()(CACHE.with(|cache| match cache.borrow().get::<Q>() {
            Some(item) => Some(item.clone()),
            None => None,
        }))
    }

    fn destroy(&mut self) {}
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
