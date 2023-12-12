use gloo::dialogs::alert;
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{log, state::State};

pub enum Msg {
    Add,
    Complete(usize),
    Remove(usize),
}

pub struct TodoApp {
    input_box: NodeRef,
    state: State,
}

impl TodoApp {
    fn save(&self) {
        let state = serde_json::to_string(&self.state).unwrap();
        let _ = LocalStorage::set("todo", state);
    }

    fn load() -> State {
        let saved_json: String = LocalStorage::get("todo").unwrap_or_default();
        let state = serde_json::from_str(&saved_json).unwrap_or_else(|_| State::new());
        state
    }
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = Self::load();
        Self {
            input_box: NodeRef::default(),
            state: state,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                let input_box_element = self.input_box.cast::<HtmlInputElement>().unwrap();
                let task_name = input_box_element.value();
                if task_name.is_empty() {
                    let _ = alert("Please enter a task");
                    return false;
                } else {
                    log!("Add task \"{}\"", task_name);
                    self.state.add(task_name);
                    input_box_element.set_value("");
                    self.save();
                    true
                }
            }
            Msg::Complete(index) => {
                log!("Toggle task {}", index);
                self.state.toggle(index);
                self.save();
                true
            }
            Msg::Remove(index) => {
                log!("Remove task {}", index);
                self.state.remove(index);
                self.save();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="todo-app">
                <div class="app-title">
                    <h2>{"Todo app"}</h2>
                </div>
                <div class="row">
                    <input ref={self.input_box.clone()} type="text" placeholder="add your tasks" />
                    <button onclick={ ctx.link().callback(|_| Msg::Add )}>{ "Add" }</button>
                </div>
                <ul>
                {
                    self.state.tasks.iter().enumerate().map(|(index, task)| {
                        let classes = if task.completed {
                            classes!("checked")
                        } else {
                            classes!("")
                        };

                        html!{
                            <li class={classes} onclick={ ctx.link().callback(move |_| Msg::Complete(index))}>
                                { format!("{}", task.name) }
                                <span onclick={ ctx.link().callback(move |event: MouseEvent| {event.stop_propagation(); Msg::Remove(index)})}>
                                    { Html::from_html_unchecked(AttrValue::from("&times;")) }
                                </span>
                            </li>
                        }
                    }).collect::<Html>()
                }
                </ul>
            </div>
        }
    }
}
