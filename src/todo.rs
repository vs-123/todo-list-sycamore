use sycamore::prelude::*;

#[component(Todo<G>)]
pub fn todo(todo_item: Signal<String>) -> View<G> {
    view! {
        li(class="todo_item") {
            input(type="checkbox", class="todo_checkbox")
            span(class="todo_title") {
                (todo_item.get().to_string())
            }
        }
    }
}