use diesel::{Insertable, Queryable};
use serde::Serialize;

use crate::schema::devs;

#[derive(Queryable, Serialize)]
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

#[derive(Insertable)]
#[table_name = "devs"]
pub struct NewDev<'a> {
    pub name: &'a str,
    pub github: &'a str,
    pub bio: &'a str,
    pub avatar_url: &'a str,
    pub techs: Vec<&'a str>,
    pub lat: &'a f32,
    pub lng: &'a f32,
}