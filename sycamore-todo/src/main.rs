use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| App(cx));
}


#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let todo_title = create_signal(cx, "");
    view! {
        cx,
        div(class="bg-black text-white h-screen w-screen p-20 overflow-auto") {
            h1(class="text-5xl") {
                "Sycamore Todo"
            }
            div(class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg") {
                //todo: bind value
                input(class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400")
                //todo: add click handler
                //todo: add conditional classes
                button(class="h-full w-40 mx-5 rounded-full") {
                    // todo: add conditional string
                    "Add todo"
                }
            }
            div(class="bg-slate-500 p-5 mt-10") {
                //todo: render by loop
            }
        }
    }
}

// todo: create a child component