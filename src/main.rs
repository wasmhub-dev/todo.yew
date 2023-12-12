mod todo;
mod utils;
mod state;
use yew::prelude::*;
use todo::TodoApp;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <TodoApp />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}