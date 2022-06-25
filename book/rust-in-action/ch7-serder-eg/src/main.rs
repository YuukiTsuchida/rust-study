use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64
}

fn main() {
    let caleabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33
    };

    let as_json = to_json(&caleabar).unwrap();
    let as_cbor = to_cbor(&caleabar).unwrap();
    let as_bincode = to_bincode(&caleabar).unwrap();

    print!("json: \n{}\n", &as_json);
    print!("cbor: \n{:?}\n", &as_cbor);
}
