use serde_yaml::Value;

use crate::structures::IrohaIterated;
use std::collections::HashMap;
// use serde::*;
// use std::fmt::{Display, Formatter};
// use serde::ser::{Serialize, Serializer, SerializeMap};

mod structures;

fn main() {
    let mut setings = HashMap::new();
    // let mut setings = BTreeMap::new();

    for i in 0..4 {
        let iroha_iter = format!("iroha{}", i.to_string());
        setings.insert(iroha_iter, value_maker(i));
    }
    for (key, value) in &setings {
        println!("{}: \n{:?}", key, value);
    }
}

fn value_maker(i: usize) -> Value {
    let entry = IrohaIterated {
        build: '.',
        image: "iroha2:dev".to_string(),
        volumes: "
        - './configs/peer:/config'
        - './:/root/soramitsu/iroha'".to_string(),
        // environment: Environment,
        ports: "
        - \"1337:1337\"
        - \"8080:8080\"
        - \"8180:8180\"".to_string(),
        init: true,
        command: "iroha --submit-genesis".to_string(),
    };
    let value = serde_yaml::to_value(&entry).unwrap();
    value
}
