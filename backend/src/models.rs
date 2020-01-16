use serde::{Deserialize, Serialize};

use super::schema::devs;

#[derive(Queryable)]
pub struct Dev {
    id: i32,
    name: String,
    github: String,
    bio: String,
    avatar_url: String,
    techs: Vec<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "devs"]
pub struct NewDev<'a> {
    pub name: &'a str,
    pub github: &'a str,
    pub bio: &'a str,
    pub avatar_url: &'a str,
    pub techs: Vec<&'a str>,
}