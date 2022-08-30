use std::iter::Map;
// use std::collections::BTreeMap;
// use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IrohaIterated {
    build: char,
    image: String,
    volumes: String,
    environment: Environment,
    ports: String,
    init: bool,
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    empty_field: String,
}