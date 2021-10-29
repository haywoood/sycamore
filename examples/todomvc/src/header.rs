use sycamore::{
    prelude::*,
    context::use_context,
    reactive::create_context_scope,
};
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

use crate::AppState;

#[component(Header<G>)]
pub fn header() -> Template<G> {
    let app_state = use_context::<AppState>();
    let value = Signal::new(String::new());

    let handle_submit = cloned!((app_state, value) => move |event: Event| {
        // create_context_scope workaround for https://github.com/sycamore-rs/sycamore/issues/282
        create_context_scope(app_state.clone(), cloned!((app_state, value, event) => move || {
            let event: KeyboardEvent = event.unchecked_into();

            if event.key() == "Enter" {
                let mut task = value.get().as_ref().clone();
                task = task.trim().to_string();

                if !task.is_empty() {
                    app_state.add_todo(task);
                    value.set("".to_string());
                }
            }
        }))
    });

    template! {
        header(class="header") {
            h1 { "todos" }
            input(class="new-todo",
                placeholder="What needs to be done?",
                bind:value=value,
                on:keyup=handle_submit,
            )
        }
    }
}
