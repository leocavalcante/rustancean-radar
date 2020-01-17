use serde_json::json;
use stdweb::web::event::IEvent;
use yew::{Component, ComponentLink, Html, html};
use yew::events::{InputData, SubmitEvent};
use yew::format::Json;
use yew::services::{console::ConsoleService, fetch::FetchService};
use yew::services::fetch::{Request, Response};

use crate::{Dev, NewDev};

#[derive(Debug)]
pub struct DevForm {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    github: String,
}

pub enum Msg {
    Submit(SubmitEvent),
    UpdateGitHub(String),
    DevAdded,
    AddError,
}

impl Component for DevForm {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DevForm {
            link,
            fetch_service: FetchService::new(),
            github: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit(event) => {
                event.prevent_default();

                let request = Request::post("http://localhost:3333/devs")
                    .header("Content-Type", "application/json")
                    .body(NewDev {
                        github: "".to_string(),
                        techs: "".to_string(),
                        lat: 0.0,
                        lng: 0.0,
                    })
                    .expect("Failed to build request.");

                let callback = self.link.callback(|response: Response<Json<Result<String, failure::Error>>>| {
                    if response.status().is_success() {
                        Msg::DevAdded
                    } else {
                        Msg::AddError
                    }
                });

                self.fetch_service.fetch(request, callback);
                false
            },
            Msg::UpdateGitHub(github) => {
                self.github = github;
                false
            },
            _ => false
        }
    }

    fn view(&self) -> Html {
        html! {
            <form onsubmit=self.link.callback(|e| Msg::Submit(e))>
                <div class="input-block">
                    <label for="github">{"Usuário do Github"}</label>
                    <input
                        name="github"
                        id="github"
                        required={true}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateGitHub(e.value))
                    />
                </div>

                <div class="input-block">
                    <label for="techs">{"Tecnologias"}</label>
                    <input
                        name="techs"
                        id="techs"
                        required={true}
                    />
                </div>

                <div class="input-group">
                    <div class="input-block">
                        <label for="latitude">{"Latitude"}</label>
                        <input
                            type="number"
                            name="latitude"
                            id="latitude"
                            required={true}
                        />
                    </div>

                    <div class="input-block">
                        <label for="logitude">{"Logitude"}</label>
                        <input
                            type="number"
                            name="logitude"
                            id="logitude"
                            required={true}
                        />
                    </div>
                </div>

                <button type="submit" class="submit">{"Salvar"}</button>
            </form>
        }
    }
}