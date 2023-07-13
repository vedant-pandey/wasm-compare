use leptos::{
    component, create_signal, event_target_value, view, For, IntoView, Scope,
    SignalUpdate,
};

struct TodoItem {
    pub id: u32,
    pub title: String,
}

impl Clone for TodoItem {
    fn clone(&self) -> Self {
        TodoItem {
            id: self.id.clone(),
            title: self.title.clone(),
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.id = source.id.clone();
        self.title = source.title.clone();
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let todos = vec![
        TodoItem {
            id: 1,
            title: "Todo 1".to_string(),
        },
        TodoItem {
            id: 2,
            title: "Todo 2".to_string(),
        },
        TodoItem {
            id: 3,
            title: "Todo 3".to_string(),
        },
    ];

    let (last_id, set_last_id) = create_signal(cx, todos.len() as u32);
    let (title, set_title) = create_signal(cx, "".to_string());
    let (todos, set_todos) = create_signal(cx, todos);

    let add_todo = move |_| {
        set_last_id.update(move |last_id| {
            *last_id += 1;
        });
        let todo = TodoItem {
            title: title(),
            id: last_id(),
        };
        set_todos.update(move |todos| (*todos).push(todo));
        set_title("".to_string());
    };

    let delete_me = move |id: u32| {
        set_todos.update(move |todos| {
            todos
                .iter()
                .position(|val| val.id == id)
                .and_then(move |k| {
                    todos.remove(k);
                    Some(())
                });
        });
    };

    view! { cx,
        <div class="bg-black text-white h-screen w-screen p-20 overflow-auto">
            <h1 class="text-5xl">"Leptos todo"</h1>
            <div class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg">
                <input
                    class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400"
                    on:input=move |ev| {
                        set_title(event_target_value(&ev))
                    }
                    prop:value=title />
                <button
                    on:click=add_todo
                    class="h-full w-40 mx-5 rounded-full"
                    class=("bg-yellow-500", move || title().len() == 0)
                    class=("bg-green-500", move || title().len() > 0)
                    disabled=move || title().len() == 0
                >
                    {move || {
                        match title().len() == 0 {
                            true => "Add a title",
                            false => "Add a todo",
                        }
                    }}
                </button>
            </div>
            <div class="bg-slate-500 p-5 mt-10">
                <For
                    each = todos
                    key = |todo| todo.id
                    view = move |cx, todo| {
                        view! { cx,
                            <TodoComponent
                                todo={todo}
                                delete_handler=delete_me
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn TodoComponent<F>(cx: Scope, todo: TodoItem, delete_handler: F) -> impl IntoView
where
    F: Fn(u32) + 'static,
{
    let (disabled, set_disabled) = create_signal(cx, true);
    let toggle_edit = move |_| set_disabled.update(|disabled| *disabled = !(*disabled));
    let delete_me = move |_| delete_handler(todo.id);
    view! { cx,
        <div class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg">
        <input
            class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400"
            value={todo.title}
            disabled={disabled}
        />
        <button
            class="h-full w-40 mx-5 rounded-full"
            class=("bg-yellow-500", move || disabled())
            class=("bg-green-500", move || !disabled())
            on:click=toggle_edit
        >
            {move || {
                match disabled() {
                    true => "Edit",
                    false => "Done",
                }
            }}
        </button>
        <button on:click=delete_me class="bg-red-500 h-full w-40 mx-5 rounded-full" >
            "Delete"
        </button>
        </div>
    }
}
