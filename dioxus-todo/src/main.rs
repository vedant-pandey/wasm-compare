#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
struct TodoItem {
    pub id: u32,
    pub title: String,
}

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let todo_title = use_state(cx, || "Todo title".to_string());
    let todo_list = vec![
        TodoItem {
            id: 1,
            title: "Todo 1".into(),
        },
        TodoItem {
            id: 2,
            title: "Todo 2".into(),
        },
        TodoItem {
            id: 3,
            title: "Todo 3".into(),
        },
    ];
    let todos = use_ref(cx, || todo_list);
    let mut last_todo_id = use_state(cx, || todos.read().len() as u32);
    cx.render(rsx! {
        link { rel: "stylesheet", href: "output.css" },
        div {
            class:"bg-black text-white h-screen w-screen p-20 overflow-auto",
            h1 {
                class:"text-5xl",
                "Dioxus todo",
            },
            div {
                class: "flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg",
                input {
                    class: "text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400",
                    value: "{todo_title}",
                    oninput: move |event| {
                        todo_title.set(event.value.clone());
                    }
                },
                button {
                    onclick: move |_| {
                        last_todo_id += 1;
                        todos.write().push(TodoItem { id: *last_todo_id.get(), title: todo_title.get().clone() });
                        todo_title.set("".to_string());
                    },
                    disabled: todo_title.len() == 0,
                    class: match todo_title.len() == 0 {
                        true => "bg-yellow-500 h-full w-40 mx-5 rounded-full",
                        false => "bg-green-500 h-full w-40 mx-5 rounded-full",
                    },
                    match todo_title.len() == 0 {
                        true => "Add a title",
                        false => "Add todo",
                    },
                },
            },
            div {
                todos.read().iter().map(|todo| {
                    rsx!(TodoComponent {
                        key: "{todo.id}",
                        todo: todo.clone(),
                        delete_handler: move |todo_id| {
                            let mut todos_mut_lock = todos.write();
                            todos_mut_lock.iter().position(|val| val.id == todo_id).and_then(move |k| {
                                todos_mut_lock.remove(k);
                                Some(())
                            });
                        }
                    })
                })
            },
        }
    })
}

#[inline_props]
fn TodoComponent<F>(cx: Scope, todo: TodoItem, delete_handler: F) -> Element
where
    F: Fn(u32),
{
    let disabled = use_state(cx, || true);
    let title = use_state(cx, || todo.title.clone());

    cx.render(rsx! {
        div {
            class: "flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg",
            input {
                class: "text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400",
                value: "{title}",
                disabled: if *disabled.get() { "true" } else { "false" },
                oninput: move |event| {
                    title.set(event.value.clone());
                }
            },
            button {
                class: if **disabled {
                    "bg-yellow-500 h-full w-40 mx-5 rounded-full"
                } else {
                    "bg-green-500 h-full w-40 mx-5 rounded-full"
                },
                onclick: move |_| {
                    disabled.set(!*disabled.get());
                },
                if **disabled {
                    "Edit"
                } else {
                    "Done"
                },
            },
            button {
                class: "bg-red-500 h-full w-40 mx-5 rounded-full",
                onclick: move |_| {
                    delete_handler(todo.id);
                },
                "Delete",
            },
        }
    })
}
