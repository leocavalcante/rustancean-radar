use yew::{Component, ComponentLink, Html, html};

pub struct DevForm;

impl Component for DevForm {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DevForm {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <form>
                <div class="input-block">
                    <label for="github_username">{"Usuário do Github"}</label>
                    <input
                        name="github_username"
                        id="github_username"
                        required={true}
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