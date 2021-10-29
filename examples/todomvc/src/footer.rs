use sycamore::context::use_context;
use sycamore::prelude::*;

use crate::{AppState, Filter};

#[component(FilterComponent<G>)]
fn filter_component(filter: Filter) -> Template<G> {
    template! {
        li {
            a(
                class="",
                href=filter.url(),
            ) {
                (format!("{:?}", filter))
            }
        }
    }
}

#[component(Footer<G>)]
pub fn footer() -> Template<G> {
    let app_state = use_context::<AppState>();

    let items_text = cloned!((app_state) => move || {
        match app_state.todos_left() {
            1 => "item",
            _ => "items"
        }
    });

    let has_completed_todos = create_selector(cloned!((app_state) => move || {
        app_state.todos_left() < app_state.todos.get().len()
    }));

    template! {
        footer(class="footer") {
            span(class="todo-count") {
                strong { (app_state.todos_left()) }
                span { " " (items_text()) " left" }
            }
            ul(class="filters") {
                FilterComponent(Filter::All)
                FilterComponent(Filter::Active)
                FilterComponent(Filter::Completed)
            }

            (if *has_completed_todos.get() {
                template! {
                    button(class="clear-completed") {
                        "Clear completed"
                    }
                }
            } else {
                Template::empty()
            })
        }
    }
}
