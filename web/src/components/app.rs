use yew::{Component, ComponentLink, Html, html};

use crate::components::dev_form::DevForm;
use crate::components::dev_item::DevItem;
use crate::Dev;

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
                        <DevItem dev={Dev {
                            avatar_url: "https://avatars2.githubusercontent.com/u/183722?s=460&v=4".to_string(),
                            github: "leocavalcante".to_string(),
                            name: "Leo Cavalcante".to_string(),
                            bio: "Tralala".to_string(),
                            techs: vec!["rust".to_string()],
                        }} />
                    </ul>
                </main>
            </div>
        }
    }
}