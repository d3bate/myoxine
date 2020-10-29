/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/
//! Myoxine is a GraphQL runtime for Rust. It's designed with Yew and WebAssembly in mind and is
//! intended to make complex applications easy to build and scale. It's currently experimental and
//! hasn't been used in the context of a serious application, but hopefully that will change soon.

pub use yew;

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

pub mod cache;
pub mod network;
pub mod objects;
pub mod query;
pub mod query_provider;

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
    /// The unique ID for this callback. This is used to remove the callback when a component
    /// unmounts.
    callback_id: i32,
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
    pub fn register_callback<Q>(&mut self, callback: Callback<Q>, event: Event) -> i32
    where
        Q: Query + 'static,
    {
        let id;
        if let Some(item) = self
            .callbacks
            .iter_mut()
            .find(|key| key.0 == &TypeId::of::<Q>())
        {
            item.1.push(CacheCallback {
                fires_on: event,
                callback: callback.reform(|item: Box<dyn Any>| *item.downcast::<Q>().unwrap()),
                callback_id: item.1.len() as i32,
            });
            id = item.1.len() as i32;
        } else {
            self.callbacks.insert(TypeId::of::<Q>(), {
                let mut vec = Vec::new();
                vec.push(CacheCallback {
                    fires_on: event,
                    callback: callback.reform(|any: Box<dyn Any>| *any.downcast::<Q>().unwrap()),
                    callback_id: 0,
                });
                vec
            });
            id = 0;
        }
        id
    }
    /// Removes any callbacks subscribing to the results of a query of type `Q`.
    pub fn deregister_callback<Q>(&mut self, callback_id: i32)
    where
        Q: Query + 'static,
    {
        for item in &mut self.callbacks {
            if *item.0 == TypeId::of::<Q>() {
                let mut to_remove = vec![];
                for (i, callback) in item.1.iter().enumerate() {
                    if callback.callback_id == callback_id {
                        to_remove.push(i)
                    }
                }
                for idx in to_remove {
                    item.1.remove(idx);
                }
            }
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
    /// Retrieve an item from the cache.
    pub fn get<Q>(&self) -> Option<&Q>
    where
        Q: Query + 'static,
    {
        self.map.get::<Q>()
    }
    pub fn update<Q>(&mut self, new_value: &Q)
    where
        Q: Query + 'static + Clone,
    {
        // todo: look for parent/child query types
        // probably needs some unsafe type dynamic type manipulation
        let item = self.map.get_mut::<Q>().unwrap();
        *item = new_value.clone();
        for callback_list in self.callbacks.get(&TypeId::of::<Q>()) {
            for callback in callback_list {
                callback.callback.emit(Box::new(item.clone()));
            }
        }
    }
}

/// This trait marks a trait as a GraphQL query. There are a couple of parts to this.
///
/// The first thing of note is that this trait requires that `Deserialize` is implemented on this
/// trait. This implementation is required so that any item implementing `Query` can be decoded from
/// a server. If your query returns a list of items, you should wrap `Vec` in the item on which
/// `Query` is implemented (e.g. `struct List(Vec<Inner>)`)
pub trait Query: for<'de> serde::Deserialize<'de> {
    fn raw_query() -> String;
    /// If this query returns only a subset of available fields on an item, Myoxine needs a way to
    /// retrieve the type of the parent. This allows the cache to quickly upgrade and downgrade
    /// items as necessary.
    fn parent() -> Option<TypeId>;
}

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
    callback_ids: Vec<i32>,
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
        let callback = CACHE.with(|cache| {
            cache
                .borrow_mut()
                .register_callback(link.callback(|q| Self::Message::Update(q)), Event::Updated)
        });
        Self {
            callback_ids: vec![callback],
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

    fn destroy(&mut self) {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            for id in &self.callback_ids {
                cache.deregister_callback::<Q>(*id);
            }
        })
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
