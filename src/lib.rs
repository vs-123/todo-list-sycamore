use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Event, KeyboardEvent};
use wasm_bindgen::JsCast;

mod todo;
use todo::Todo;

#[component(App<G>)]
fn app() -> View<G> {
    let todos = Signal::new(Vec::<String>::new());
    let new_todo_name = Signal::new(String::new());
    let add_todo = cloned!((new_todo_name, todos) => move |e: Event| {
        let event: KeyboardEvent = e.unchecked_into();

        if event.key() == "Enter" {
            todos.clone().set(
                (*todos.get())
                    .iter()
                    .chain(vec![new_todo_name.clone().get().to_string()].iter())
                    .map(|s| (*s).clone())
                    .collect::<_>()
            );
            new_todo_name.set(String::new())
        }
    });

    view! {
        style {
        ("
            body {
                font-family: sans-serif;
            }

            #new_todo_input {
                font-size: 1.5em;
            }

            #todo_list {
                list-style: none;
            }

            .todo_title {
                font-size: 1.5em;
            }

            .todo_checkbox {
                width: 15px;
                height: 15px;
            }
        ")
        }
        div {
            input(type="text", id="new_todo_input", bind:value=new_todo_name.clone(), on:keyup=add_todo) 

            ul(id="todo_list") {
                Keyed(KeyedProps {
                    iterable: todos.handle(),

                    template: |todo| view! {
                        Todo(Signal::new(todo))
                    },

                    key: |todo| todo.len(),
                })
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    sycamore::render(|| view! { App() })
}
