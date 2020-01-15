use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DevRequest {
    github: String,
}