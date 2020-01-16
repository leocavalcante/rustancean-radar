use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[table_name = "devs"]
pub struct Dev {
    id: i32,
    name: String,
    github: String,
    bio: String,
    avatar_url: String,
    techs: Vec<String>,
}