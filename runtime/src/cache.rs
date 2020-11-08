/*
Built with love and the hope that you'll use this software for good by d3bate.

This file is distributed subject to the terms of the Affero General Public License.
A copy of the license can be found at the root of this Git repository.
*/

use crate::objects::Object;
use std::cell::RefCell;
use std::thread::LocalKey;
use std::{any::Any, any::TypeId, rc::Rc};
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

pub trait Cache: 'static + Sized {
    /// Caches an item.
    fn cache<O>(&mut self, item: O)
    where
        O: Object + 'static;
    fn retrieve<O>(&self, id: &crate::Id) -> Option<Rc<O>>
    where
        O: Object + 'static;
    fn subscribe<O>(
        &mut self,
        selector: &'static dyn Fn(&O) -> bool,
        callback: Callback<O>,
        event: Event,
    ) -> u64
    where
        O: Object + Clone + 'static;
    fn unsubscribe(&mut self, id: u64);
    /// Evicts an item from the cache.
    fn remove<O>(&mut self, object: &crate::Id)
    where
        O: Object;
    fn local_key() -> &'static LocalKey<RefCell<Self>>;
}

thread_local! {
    pub static VANILLA_CACHE: RefCell<VanillaCache> = RefCell::new(VanillaCache::new())
}

pub struct VanillaCache {
    items: Vec<(TypeId, Rc<dyn Any>)>,
    subscriptions: Vec<(
        u64,
        Event,
        TypeId,
        Box<dyn Fn(Rc<dyn Any>) -> bool>,
        Callback<Rc<dyn Any>>,
    )>,
    subscription_counter: u64,
}

impl VanillaCache {
    fn new() -> Self {
        Self {
            items: vec![],
            subscriptions: vec![],
            subscription_counter: 0,
        }
    }
}

impl Cache for VanillaCache {
    fn cache<O>(&mut self, item: O)
    where
        O: Object + 'static,
    {
        let position = self
            .items
            .iter()
            .filter(|(type_id, _)| type_id == &TypeId::of::<O>())
            .position(|(_, cmp_item)| cmp_item.downcast_ref::<O>().unwrap().id() == item.id());
        if let Some(position) = position {
            *self.items.get_mut(position).unwrap() = (TypeId::of::<O>(), Rc::new(item));
        } else {
            self.items.push((TypeId::of::<O>(), Rc::new(item)));
        }
        let item = self.items.get(self.items.len() - 1).unwrap();
        for _ in self
            .subscriptions
            .iter()
            .filter(|subscription| subscription.2 == TypeId::of::<O>())
            .map(|subscription| {
                subscription.4.emit(item.1.clone());
            })
        {}
    }

    fn retrieve<O>(&self, id: &crate::Id) -> Option<Rc<O>>
    where
        O: Object + 'static,
    {
        self.items
            .iter()
            .filter(|item| item.0 == TypeId::of::<O>())
            .find(|item| item.1.clone().downcast::<O>().unwrap().id() == id)
            .map(|item| item.1.clone().downcast::<O>().unwrap())
    }

    fn subscribe<O>(
        &mut self,
        selector: &'static dyn Fn(&O) -> bool,
        callback: Callback<O>,
        event: Event,
    ) -> u64
    where
        O: Object + Clone + 'static,
    {
        let x = self.subscription_counter.clone();
        self.subscriptions.push((
            x,
            event,
            TypeId::of::<O>(),
            Box::new(move |input: Rc<dyn Any>| selector(input.downcast_ref::<O>().unwrap())),
            callback.reform(|any: Rc<dyn Any>| any.downcast_ref::<O>().map(Clone::clone).unwrap()),
        ));
        self.subscription_counter += 1;
        x
    }

    fn unsubscribe(&mut self, id: u64) {
        self.subscriptions
            .iter()
            .position(|item| item.0 == id)
            .expect(
                "attempted to unsubscribe a subscription which either has already been \
        unsubscribed or did not exist in the first place",
            );
    }

    fn remove<O>(&mut self, object: &crate::Id)
    where
        O: Object,
    {
        let position = self
            .items
            .iter()
            .filter(|(type_id, _)| type_id == &TypeId::of::<O>())
            .position(|(_, item)| item.downcast_ref::<O>().unwrap().id() == object);
        if let Some(position) = position {
            let item = self.items.remove(position);
            for relevant_subscription in
                self.subscriptions
                    .iter()
                    .filter(|(_, event, _, _, _)| match event {
                        Event::Delete => true,
                        _ => false,
                    })
            {
                relevant_subscription.4.emit(item.1.clone());
            }
        }
    }
    fn local_key() -> &'static LocalKey<RefCell<Self>> {
        &VANILLA_CACHE
    }
}
