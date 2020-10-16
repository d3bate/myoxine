/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

use crate::objects::Object;
use std::{collections::HashMap, hash::Hash};
use thiserror::Error as ThisError;
use yew::Callback;

#[derive(ThisError, Debug)]
pub enum CacheError {
    #[error("the provided item was not found in the cache")]
    NotFound,
}

pub enum Event {
    Create,
    Update,
    Delete,
}

pub struct Subscription<O: 'static> {
    callback: Callback<O>,
    event: Event,
    selector: &'static dyn Fn(O) -> bool,
}

impl<O> Subscription<O> {
    fn new(callback: Callback<O>, event: Event, selector: &'static dyn Fn(O) -> bool) -> Self {
        Self {
            callback,
            event,
            selector,
        }
    }
}

pub trait Cache<O: Object>: 'static {
    /// Caches an item.
    fn cache(&mut self, item: O) -> Result<(), CacheError>;
    fn subscribe(
        &mut self,
        selector: &'static dyn Fn(O) -> bool,
        callback: Callback<O>,
        event: Event,
    ) -> u64;
    fn unsubscribe(&mut self, id: u64);
    /// Evicts an item from the cache.
    fn remove(&mut self, object: &O::Id);
}

/// The standard cache, provided by default.
///
/// This is just a thin wrapper around a `HashMap`; more elaborate caches are planned for
/// including time-based caches and the like).
pub struct VanillaCache<O>
where
    O: Object,
    O::Id: PartialEq,
{
    items: HashMap<O::Id, O>,
    subscriptions: Vec<(u64, Subscription<O>)>,
    subscription_counter: u64,
}

impl<O> Cache<O> for VanillaCache<O>
where
    O: Object,
    O::Id: PartialEq + Eq + Hash,
{
    fn cache(&mut self, item: O) -> Result<(), CacheError> {
        // I was amazed that this operation could not fail (maybe I've done something wrong)
        let cache_item = self.items.get_mut(&item.id());
        match cache_item {
            Some(some_item) => {
                *some_item = item;
            }
            None => {
                self.items.insert(item.id(), item);
            }
        }
        Ok(())
    }
    fn subscribe(
        &mut self,
        selector: &'static dyn Fn(O) -> bool,
        callback: Callback<O>,
        event: Event,
    ) -> u64 {
        self.subscriptions.push((
            self.subscription_counter,
            Subscription::new(callback, event, selector),
        ));
        let subscription_counter = self.subscription_counter;
        self.subscription_counter += 1;
        subscription_counter
    }
    fn unsubscribe(&mut self, id: u64) {
        let location = self
            .subscriptions
            .iter()
            .enumerate()
            .filter(|item| (item.1).0 == id)
            .next()
            .unwrap()
            .0;
        self.subscriptions.remove(location);
    }
    fn remove(&mut self, item: &O::Id) {
        self.items.remove(item);
    }
}

#[cfg(test)]
/// These tests cannot be written yet before the `Object` derive macro has been completed.
mod vanilla_cache_test {
    #[test]
    fn test_cache() {
        todo!()
    }
    #[test]
    fn test_retrieve() {
        todo!()
    }
    #[test]
    fn test_subscribe() {
        todo!()
    }
    #[test]
    fn test_unsubscribe() {
        todo!()
    }
}
