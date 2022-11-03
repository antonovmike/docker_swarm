use crate::structures::{IrohaIterated, Environment};
use std::collections::HashMap;

mod structures;

fn main() {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("config.yml")
        .expect("Couldn't open file");

    let mut setings = HashMap::new();

    for i in 0..4 {
        let value: Vec<u8> = serializer(i);
        let serde_content = value
            .into_iter()
            .take_while(|&x| x != 0)
            .collect::<Vec<_>>();
        let serde_data = String::from_utf8(serde_content).expect("Invalid utf8 message");

        setings.insert(i, serde_data);
    }
    for (key, value) in &setings {
        println!("{}: \n{}", key, value);
    }

    serde_yaml::to_writer(file, &setings).unwrap();
}

fn environment_data(iroha_iter: usize) -> Environment {
    let envir = Environment {
        TORII_P2P_ADDR:      format!("iroha{}:1337", iroha_iter),
        TORII_API_URL:       format!("iroha{}:8080", iroha_iter),
        TORII_TELEMETRY_URL: format!("iroha{}:8180", iroha_iter),
        IROHA_PUBLIC_KEY:       "EMPTY".to_string(),
        IROHA_PRIVATE_KEY:      "EMPTY".to_string(),
        SUMERAGI_TRUSTED_PEERS: "EMPTY".to_string(),
    };
    envir
}

fn serializer(iroha_iter: usize) -> Vec<u8> {
    let irohaiter = IrohaIterated {
        build: '.',
        image: "iroha2:dev".to_string(),
        volumes: "- './configs/peer:/config'\n- './:/root/soramitsu/iroha'".to_string(),
        environment: environment_data(iroha_iter),
        ports: "- \"1337:1337\"\n- \"8080:8080\"\n- \"8180:8180\"".to_string(),
        init: true,
        command: "iroha --submit-genesis".to_string(),
    };

    let serialized = serde_yaml::to_string(&irohaiter)
        .unwrap()
        .clone()
        .into_bytes();

    serialized
}
