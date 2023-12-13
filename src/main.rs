mod todo;
mod state;
use yew::prelude::*;
use todo::TodoApp;
use gloo::console::log;

#[function_component]
fn App() -> Html {
    log!("Welcome to wasm(yew) world");
    html! {
        <div class="container">
            <TodoApp />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}