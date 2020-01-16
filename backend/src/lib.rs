#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

pub mod schema;
pub mod models;
pub mod application;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}