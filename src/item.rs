use serde::{Deserialize, Serialize};
use gloo_console::log;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            completed: false,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub item: TodoItem,
    pub on_delete: Callback<Uuid>,
}

#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    let item = use_state(|| props.item.clone());
    let id = props.item.id;
    let delete = props.on_delete.clone();
    let state = item.clone();

    let toggle_done = Callback::from(move |_: Event| {
        let mut state_item = (*state).clone();
        state_item.completed = !state_item.completed;

        state.set(state_item);
    });

    let delete = Callback::from(move |_| {
        delete.emit(id);
    });

    html! {
        <div class={
            classes!(
            "item",
            (*item).completed.then(|| "is-done")
        )
        }>
            <input onchange={toggle_done} type="checkbox" class="checkbox" />
            <p class="item-text">{&props.item.text}</p>
            <button class="delete" onclick={delete}>{"Delete"}</button>
        </div>
    }
}
