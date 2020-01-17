use yew::{Component, ComponentLink, Html, html, Properties};

use crate::Dev;

#[derive(Properties, PartialEq, Clone)]
pub struct DevItemProps {
    #[props(required)]
    pub dev: Dev
}

pub struct DevItem {
    props: DevItemProps
}

impl Component for DevItem {
    type Message = ();
    type Properties = DevItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DevItem { props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <li class="dev-item">
                <header>
                    <img src=&self.props.dev.avatar_url alt=&self.props.dev.name />
                    <div class="user-info">
                        <strong>{&self.props.dev.name}</strong>
                        <span>{&self.props.dev.techs.join(", ")}</span>
                    </div>
                </header>
                <p>{&self.props.dev.bio}</p>
                <a href=format!("https://github.com/{}", &self.props.dev.github)>{"Acessar perfil no Github"}</a>
            </li>
        }
    }
}