#![recursion_limit = "640"]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq)]
pub struct Dev {
    id: i32,
    name: String,
    github: String,
    bio: String,
    avatar_url: String,
    techs: Vec<String>,
    lat: f32,
    lng: f32,
}

#[derive(Serialize)]
pub struct NewDev {
    github: String,
    techs: String,
    lat: f32,
    lng: f32,
}

impl Into<Result<String, failure::Error>> for NewDev {
    fn into(self) -> Result<String, failure::Error> {
        Ok("".to_owned())
    }
}

pub mod components;