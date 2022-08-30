use std::collections::HashMap;

fn main() {
    let mut setings = HashMap::new();

    for i in 0..4 {
        let iroha_iter = format!("iroha{}", i.to_string());
        setings.insert(iroha_iter, value_maker(i));
    }
    for (key, value) in &setings {
        println!("{}: {}", key, value);
    }
}

fn value_maker(i: usize) -> String {
    i.to_string()
}