use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    AddTask
}

pub struct TodoApp {
    input_box: NodeRef,
    tasks: Vec<(bool, String)>,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_box: NodeRef::default(),
            tasks: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTask => {
                let task_name = self.input_box.cast::<HtmlInputElement>().unwrap().value();
                self.tasks.push((false, task_name));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_task_button_onclick = ctx.link().callback(|_| {
            Msg::AddTask
        });

        html! {
            <div class="todo-app">
                <div class="app-title">
                    <h2>{"Todo app"}</h2>
                    <i class="fa-solid fa-book-bookmark"></i>
                </div>
                <div class="row">
                    <input ref={self.input_box.clone()} type="text" placeholder="add your tasks" />
                    <button onclick={ add_task_button_onclick }>{ "Add" }</button>
                </div>
                <ul>
                {
                    &self.tasks.clone().into_iter().map(|task| {
                        let completed = task.0;
                        let task_name = task.1;
                        let classes = if completed {
                            classes!("checked")
                        } else {
                            classes!("")
                        };

                        html!{
                            <li class={classes}>
                                { format!("{}", task_name) }
                                <span>{ Html::from_html_unchecked(AttrValue::from("&times;")) }</span>
                            </li>
                        }
                    }).collect::<Html>()
                }
                </ul>
            </div>
        }
    }
}