use crate::structures::IrohaIterated;
use std::collections::HashMap;

mod structures;

fn main() {
    let mut setings = HashMap::new();

    for i in 0..4 {
        let iroha_iter = format!("iroha{}", i.to_string());
        
        let value: Vec<u8> = value_maker();
        let serde_content = value
            .into_iter()
            .take_while(|&x| x != 0)
            .collect::<Vec<_>>();
        let serde_message = String::from_utf8(serde_content).expect("Invalid utf8 message");

        setings.insert(iroha_iter, serde_message);
    }
    for (key, value) in &setings {
        println!("{}: \n{}", key, value);
    }
}

fn value_maker() -> Vec<u8> {
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

    let serialized = serde_yaml::to_string(&entry)
        .unwrap()
        .clone()
        .into_bytes();

    serialized
}
