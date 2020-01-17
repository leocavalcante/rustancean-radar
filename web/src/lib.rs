#![recursion_limit = "640"]

#[derive(Clone, PartialEq)]
pub struct Dev {
    avatar_url: String,
    name: String,
    techs: Vec<String>,
    bio: String,
    github: String,
}

pub mod components;