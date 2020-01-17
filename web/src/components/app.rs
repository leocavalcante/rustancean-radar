use yew::{Component, ComponentLink, Html, html};

use crate::components::dev_form::DevForm;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app">
                <aside>
                    <strong>{"Cadastrar"}</strong>
                    <DevForm />
                </aside>
                <main>
                    <ul>
                    </ul>
                </main>
            </div>
        }
    }
}