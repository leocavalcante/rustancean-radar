use diesel::{Insertable, Queryable};

table! {
    use diesel::sql_types::*;

    dev {
        id -> Integer,
        name -> VarChar,
        github -> VarChar,
        bio -> Text,
        avatar_url -> VarChar,
        techs -> Array<VarChar>,
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "dev"]
pub struct Dev {
    id: i32,
    name: String,
    github: String,
    bio: String,
    avatar_url: String,
    techs: Vec<String>,
}