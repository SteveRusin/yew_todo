use std::rc::Rc;

use gloo_console::log;
use uuid::Uuid;
use yew::prelude::*;

use crate::add_item::AddItem;
use crate::item::{Item, TodoItem};

#[function_component]
pub fn App() -> Html {
    let items: UseStateHandle<Vec<TodoItem>> = use_state(|| vec![]);
    let state = items.clone();

    let on_add_item = Callback::from(move |item: String| {
        let item = TodoItem::new(item);
        // log!(serde_json::to_string_pretty(&item).unwrap());
        let mut items = state.to_vec();
        items.push(item);

        state.set(items);
    });

    let state = items.clone();

    let on_delete_item = Callback::from(move |id: Uuid| {
        let mut items = state.to_vec();
        items.retain(|item| item.id != id);

        state.set(items);
    });

    html! {
        <div class="wrapper">
            <h1 class="title">{"Rust Todo App"}</h1>
            <AddItem {on_add_item}/>
          {
            items.iter().map(|item: &TodoItem| {

            html!{ <Item item={item.clone()} on_delete={on_delete_item.clone()} /> }
           }).collect::<Html>()
          }
        </div>
    }
}
