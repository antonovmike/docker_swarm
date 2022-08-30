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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Environment {
//     empty_field: String,
// }