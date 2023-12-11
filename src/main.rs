mod todo;
mod item;
mod utils;
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