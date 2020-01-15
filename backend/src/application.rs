use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DevRequest {
    pub github: String,
    pub techs: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub name: String,
    pub avatar_url: String,
    pub bio: String,
}