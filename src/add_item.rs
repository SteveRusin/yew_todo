use gloo_console::log;
use wasm_bindgen::JsCast;
use wasm_logger;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_add_item: Callback<String>,
}

#[function_component]
pub fn AddItem(props: &Props) -> Html {
    let input_value = use_state(|| "".to_owned());

    let oninput = {
        let state = input_value.clone();

        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();

            state.clone().set(input.value());
        })
    };

    let emit_add_item = {
        let state = input_value.clone();
        let on_add_item = props.on_add_item.clone();

        move || {
            on_add_item.emit((*state).clone());
            state.set("".to_owned());
        }
    };

    let add_item = {
        let emit = emit_add_item.clone();

        Callback::from(move |_| {
            emit();
        })
    };

    let on_enter = {
        let emit = emit_add_item.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                emit();
            }
        })
    };

    html! {
        <div class="add-item">
            <input type="text" {oninput} onkeyup={on_enter} value={(*input_value).clone()} placeholder="What needs to be done?" />
            <button class="add-btn" onclick={add_item}>{"Add"}</button>
        </div>
    }
}
