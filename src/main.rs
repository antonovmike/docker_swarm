use crate::structures::{IrohaIterated, Environment};
use std::collections::HashMap;
use serde_yaml::Error;

mod structures;

fn main() -> Result<(), Error> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("config.yml")
        .expect("Couldn't open file");

    let mut setings = HashMap::new();

    for i in 0..4 {
        let iroha_iter = format!("iroha{}", i.to_string());

        let value: Vec<u8> = value_maker(&iroha_iter)?;
        let serde_content = value
            .into_iter()
            .take_while(|&x| x != 0)
            .collect::<Vec<_>>();
        let serde_data = String::from_utf8(serde_content).expect("Invalid utf8 message");

        setings.insert(iroha_iter, serde_data);
    }
    for (key, value) in &setings {
        println!("{}: \n{}", key, value);
    }

    serde_yaml::to_writer(file, &setings)?;

    Ok(())
}

fn environment_data(iroha_iter: &String) -> Environment {
    let envir = Environment {
        TORII_P2P_ADDR:      format!("{}:1337", iroha_iter),
        TORII_API_URL:       format!("{}:8080", iroha_iter),
        TORII_TELEMETRY_URL: format!("{}:8180", iroha_iter),
        IROHA_PUBLIC_KEY:       dummy(),
        IROHA_PRIVATE_KEY:      dummy(),
        SUMERAGI_TRUSTED_PEERS: dummy(),
    };
    envir
}

fn dummy() -> String { "EMPTY".to_string() }

fn value_maker(iroha_iter: &String) -> Result<Vec<u8>, Error> {
    let irohaiter = IrohaIterated {
        build: '.',
        image: "iroha2:dev".to_string(),
        volumes: "- './configs/peer:/config'\n- './:/root/soramitsu/iroha'".to_string(),
        environment: environment_data(&iroha_iter),
        ports: "- \"1337:1337\"\n- \"8080:8080\"\n- \"8180:8180\"".to_string(),
        init: true,
        command: "iroha --submit-genesis".to_string(),
    };

    let serialized = serde_yaml::to_string(&irohaiter)?
        .clone()
        .into_bytes();

    Ok(serialized)
}
