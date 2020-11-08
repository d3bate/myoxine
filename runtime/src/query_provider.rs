use std::{marker::PhantomData, rc::Rc};

use crate::cache::Event;
use crate::cache::{Cache, VanillaCache};
use crate::network::{Network, VanillaNetwork};
use crate::objects::Object;
use serde::Deserialize;
use std::fmt::Debug;
use wasm_bindgen::__rt::core::fmt::Formatter;
use yew::prelude::*;

pub struct QueryProvider<OUT, CHILD, NETWORK = VanillaNetwork, CACHE = VanillaCache>
where
    OUT: for<'de> Deserialize<'de> + 'static + Clone + Object,
    NETWORK: Network + 'static,
    CACHE: Cache + 'static,
    CHILD: Component + Clone,
    CHILD::Properties: From<Rc<OUT>> + Debug,
{
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    subscription_id: u64,
    props: QueryProviderProps<OUT, CHILD>,
    item: Option<Rc<OUT>>,
    _out: PhantomData<OUT>,
    _network: PhantomData<NETWORK>,
    _cache: PhantomData<CACHE>,
}

impl<OUT, CHILD, NETWORK, CACHE> Debug for QueryProvider<OUT, CHILD, NETWORK, CACHE>
where
    OUT: for<'de> Deserialize<'de> + 'static + Clone + Object,
    NETWORK: Network + 'static,
    CACHE: Cache + 'static,
    CHILD: Component + Clone,
    CHILD::Properties: From<Rc<OUT>> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("QueryProvider")
    }
}

#[derive(Properties, Clone)]
pub struct QueryProviderProps<OUT, CHILD>
where
    OUT: for<'de> Deserialize<'de> + 'static + Clone,
    CHILD: Component + Clone,
    CHILD::Properties: From<Rc<OUT>> + Debug,
{
    render: Rc<dyn Fn(OUT) -> Html>,
    children: yew::ChildrenWithProps<CHILD>,
}

impl<OUT, CHILD> Debug for QueryProviderProps<OUT, CHILD>
where
    OUT: for<'de> Deserialize<'de> + 'static + Clone + Debug,
    CHILD: Component + Clone,
    CHILD::Properties: From<Rc<OUT>> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("QueryProviderProps {}")
    }
}

pub enum QueryProviderMsg {
    Update,
}

impl<OUT, CHILD, NETWORK, CACHE> Component for QueryProvider<OUT, CHILD, NETWORK, CACHE>
where
    OUT: for<'de> Deserialize<'de> + 'static + Clone + Object,
    NETWORK: Network + 'static,
    CACHE: Cache + 'static,
    CHILD: Component + Clone,
    CHILD::Properties: From<Rc<OUT>> + Debug,
{
    type Message = QueryProviderMsg;

    type Properties = QueryProviderProps<OUT, CHILD>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let subscription_id = CACHE::local_key().with(|cache| {
            let mut cache = cache.borrow_mut();

            cache.subscribe::<OUT>(
                &|&_| true,
                link.callback(|_| Self::Message::Update),
                Event::Update,
            )
        });
        Self {
            link,
            item: None,
            subscription_id,
            props,
            _out: PhantomData,
            _cache: PhantomData,
            _network: PhantomData,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Update => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(item) = &self.item {
            return html! {
                {for self.props.children.iter().map(|mut child| {
                    child.props = From::from(item.clone());
                    child
                })}
            };
        } else {
            return html! {
                <h1>{"Loading..."}</h1>
            };
        }
    }

    fn destroy(&mut self) {
        CACHE::local_key().with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.unsubscribe(self.subscription_id);
        });
    }
}
