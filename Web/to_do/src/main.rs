use yew::{ Component, NodeRef, Html, Context, html };
use web_sys::{HtmlInputElement, console};


enum Msg {
    AddOne,
}

struct Thing {
    contents: String,
}

impl Thing {
    fn new(value: &String) -> Self {
        Self {
            contents: value.to_owned(),
        }
    }
}

struct Model {
    value: Vec<Thing>,
    input_value: String,
    input_ref: NodeRef,
}

impl Component for Model {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: Vec::new(),
            input_value: "".to_owned(),
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                let input = self.input_ref.clone().cast::<HtmlInputElement>();
                console::log_1(&format!("{:?}", input.as_ref().unwrap().value()).into());
                self.value.push(Thing::new(&input.unwrap().value()));
                self.input_value = "".to_owned();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let contents = self.value.iter().map(|item| html! { <li>{item.contents.clone()}</li> }).collect::<Html>();
        html! {
            <div>
                <input ref={self.input_ref.clone()} value={self.input_value.clone()} />
                <p> {self.input_value.clone() }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <ul class="todo-list">
                    { contents }
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
