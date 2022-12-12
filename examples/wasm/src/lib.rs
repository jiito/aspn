use std::{error::Error, io::stdin};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub name: String,
    pub num: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub names: Vec<String>,
}

#[no_mangle]
pub extern "C" fn greet() {
    let input: Input = serde_json::from_reader(stdin())
        .map_err(|e| {
            eprintln!("ser: {e}");
            e
        })
        .unwrap();

    let names = vec![
        String::from("Running"),
        String::from("wasm code on"),
        input.name,
    ];

    let output = Output { names };
    let serialized = serde_json::to_string(&output)
        .map_err(|e| {
            eprintln!("de: {e}");
            e
        })
        .unwrap();

    println!("{serialized}");
}
