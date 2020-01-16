use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DevRequest {
    pub github: String,
    pub techs: String,
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub name: String,
    pub avatar_url: String,
    pub bio: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchQuery {
    pub lat: f32,
    pub lng: f32,
    pub techs: String,
}