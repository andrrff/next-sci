use yew::prelude::*;

mod bindings;

use bindings::js_caller::calculator;

enum Msg {
    AddOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        calculator();
        html! {
            <div class="calculator">
                <div class="input" id="input"></div>
                <div class="buttons">
                    <div class="operators">
                    <div>{"<"}</div>
                    <div>{">"}</div>
                    <div>{"π"}</div>
                    <div>{"log"}</div>
                    <div>{"∫"}</div>
                    <div>{"^"}</div>
                    <div>{"sin"}</div>
                    <div>{"cos"}</div>
                    </div>
                    <div class="operators">
                    <div>{"+"}</div>
                    <div>{"-"}</div>
                    <div>{"×"}</div>
                    <div>{"÷"}</div>
                    </div>
                    <div class="leftPanel">
                    <div class="numbers">
                        <div>{"7"}</div>
                        <div>{"8"}</div>
                        <div>{"9"}</div>
                    </div>
                    <div class="numbers">
                        <div>{"4"}</div>
                        <div>{"5"}</div>
                        <div>{"6"}</div>
                    </div>
                    <div class="numbers">
                        <div>{"1"}</div>
                        <div>{"2"}</div>
                        <div>{"3"}</div>
                    </div>
                    <div class="numbers">
                        <div>{"0"}</div>
                        <div>{"."}</div>
                        <div id="clear">{"C"}</div>
                    </div>
                    </div>
                    <div class="equal" id="result">{"="}</div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}