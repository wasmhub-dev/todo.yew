use yew::prelude::*;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

pub struct TodoItem;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub completed: bool,
}

pub enum Msg {
    Toggle,
    Remove,
}

impl Component for TodoItem {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                true
            }

            Msg::Remove => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|event: MouseEvent| {
            let target_element = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            let tag_name = target_element.tag_name();
            if tag_name == "LI" {
                Msg::Toggle
            } else if tag_name == "SPAN" {
                Msg::Remove
            } else {
                unreachable!()
            }
        });

        let name = ctx.props().name.clone();
        let completed = ctx.props().completed;
        let classes = if completed {
            classes!("checked")
        } else {
            classes!("")
        };

        html! {
            <li {onclick} class={classes}>
                { format!("{}", name) }
                <span>{ Html::from_html_unchecked(AttrValue::from("&times;")) }</span>
            </li>
        }
    }
}