use std::collections::{HashMap, BTreeMap};
use serde::*;
use std::fmt::{Display, Formatter};
use serde::ser::{Serialize, Serializer, SerializeMap};

mod structures;

fn main() {
    let mut setings = HashMap::new();
    // let mut setings = BTreeMap::new();

    for i in 0..4 {
        let iroha_iter = format!("iroha{}", i.to_string());
        setings.insert(iroha_iter, value_maker(i));
    }
    for (key, value) in &setings {
        println!("{}: \n{}", key, value);
    }
}

fn value_maker(i: usize) -> String {
    serde_yaml::to_string(&i).unwrap()
}
