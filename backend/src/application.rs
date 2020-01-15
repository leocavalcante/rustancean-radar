use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DevRequest {
    pub github: String,
}