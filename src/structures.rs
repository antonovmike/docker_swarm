use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IrohaIterated {
    pub build: char,
    pub image: String,
    pub volumes: String,
    // pub environment: Environment,
    pub ports: String,
    pub init: bool,
    pub command: String,
}
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub TORII_P2P_ADDR: String,
    pub TORII_API_URL: String,
    pub TORII_TELEMETRY_URL: String,
}