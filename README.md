# Myoxine

> Myoxine is distributed subject to the Mozilla Public License 2.0. Any use of this code
> may only be in compliance with the license.

```rust
use yew::prelude::*;

query!(Query, {
    user {
        id
    }
});

struct App {

}

impl Component for App {
    type Msg = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, _: Self::Msg) -> bool {
        false
    }
    fn view(&self) -> Html {
        html! {
            <QueryProvider<Query> render={move |query: Query| {
                html! {
                    <h1>{format!("User: {}", query.id)}</h1>
                }
            }}>
            </QueryProvider<Query>>
        }
    }
}
```