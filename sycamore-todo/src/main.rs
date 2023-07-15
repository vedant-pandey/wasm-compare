#[allow(non_snake_case)]
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| App(cx));
}

#[derive(Clone, PartialEq)]
struct TodoItem {
    id: u32,
    title: String,
}
 
#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let todo_title = create_signal(cx, "".to_string());
    let todos = create_signal(cx, vec![
        TodoItem {id: 1, title: "Todo 1".into()},
        TodoItem {id: 2, title: "Todo 2".into()},
        TodoItem {id: 3, title: "Todo 3".into()},
    ]);
    let last_id = create_signal(cx, todos.get().len() as u32);
    view! {
        cx,
        div(class="bg-black text-white h-screen w-screen p-20 overflow-auto") {
            h1(class="text-5xl") {
                "Sycamore Todo"
            }
            div(class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg") {
                input(class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400", bind:value=todo_title)
                button(class=(if todo_title.get().len() == 0 {
                    "bg-yellow-500 h-full w-40 mx-5 rounded-full"
                } else {
                    "bg-green-500 h-full w-40 mx-5 rounded-full"
                }),
                on:click = move |_| {
                    last_id.set(*last_id.get() + 1);
                    let mut todos_clone = (*todos.get()).clone();
                    todos_clone.push(TodoItem{id: *last_id.get(), title: (*todo_title.get()).clone()});
                    todos.set(todos_clone);
                    todo_title.set("".into());
                },
                disabled=todo_title.get().len() == 0
                ) {
                    (if todo_title.get().len() == 0 {
                        view! {cx, "Add a title"}
                    } else {
                        view! {cx, "Add a todo"}
                    })
                }
            }
            div(class="bg-slate-500 p-5 mt-10") {
                Keyed(
                    iterable=todos,
                    view = move |cx, todo| view! {
                        cx,
                        TodoComponent(todo = todo, delete_handler = move |id| {
                            let mut todos_clone = (*todos.get()).clone();
                            todos_clone.iter().position(|todo| todo.id == id).and_then(|k| {
                                todos_clone.remove(k);
                                Some(())
                            });
                            todos.set(todos_clone);
                        })
                    },
                    key = |todo| todo.id
                )
            }
        }
    }
}

#[component(inline_props)]
fn TodoComponent<'a, G: Html , F>(cx: Scope<'a>, todo: TodoItem, delete_handler: F) -> View<G>
where
    F: Fn(u32) + 'a,
{
    let disabled = create_signal(cx, true);
    let title = create_signal(cx, todo.title);
    view! {
        cx,
        div(class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg") {
            input(class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400", bind:value=title, disabled=*disabled.get())
            button(class=(if *disabled.get() {
                    "bg-yellow-500 h-full w-40 mx-5 rounded-full"
                } else {
                    "bg-green-500 h-full w-40 mx-5 rounded-full"
                }), on:click=move |_| {
                    disabled.set(!*disabled.get());
                }) {
                    (if *disabled.get() {
                        view! {cx, "Edit"}
                    } else {
                        view! {cx, "Done"}
                    })
            }
            button(class="bg-red-500 h-full w-40 mx-5 rounded-full", on:click = move |_| {
                delete_handler(todo.id);
            }) {
                "Delete"
            }
        }
    }
}
