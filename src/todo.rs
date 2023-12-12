use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::state::State;

pub enum Msg {
    Add,
    Complete(usize),
    Remove(usize),
}

pub struct TodoApp {
    input_box: NodeRef,
    state: State,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_box: NodeRef::default(),
            state: State::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                let input_box_element = self.input_box.cast::<HtmlInputElement>().unwrap();
                let task_name = input_box_element.value();
                self.state.add(task_name);
                input_box_element.set_value("");
                true
            }
            Msg::Complete(index) => {
                self.state.toggle(index);
                true
            }
            Msg::Remove(index) => {
                self.state.remove(index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="todo-app">
                <div class="app-title">
                    <h2>{"Todo app"}</h2>
                    <i class="fa-solid fa-book-bookmark"></i>
                </div>
                <div class="row">
                    <input ref={self.input_box.clone()} type="text" placeholder="add your tasks" />
                    <button onclick={ ctx.link().callback(|_| Msg::Add )}>{ "Add" }</button>
                </div>
                <ul>
                {
                    &self.state.tasks.iter().enumerate().map(|(index, task)| {
                        let classes = if task.completed {
                            classes!("checked")
                        } else {
                            classes!("")
                        };

                        html!{
                            <li class={classes} onclick={ ctx.link().callback(move |_| Msg::Complete(index))}>
                                { format!("{}", task.name) }
                                <span onclick={ ctx.link().callback(move |event: MouseEvent| {event.stop_propagation(); Msg::Remove(index)})}>{ Html::from_html_unchecked(AttrValue::from("&times;")) }</span>
                            </li>
                        }
                    }).collect::<Html>()
                }
                </ul>
            </div>
        }
    }
}