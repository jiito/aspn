use std::io::Read;
use wasi_common::pipe::{ReadPipe, WritePipe};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub name: String,
    pub num: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub names: Vec<String>,
}

pub fn run(path: &str) -> Result<Output> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.

    // TAKEN FROM https://github.com/pmalmgren/wasi-data-sharing/blob/shared-stdio-demo/examples/wasi/main.rs
    let input = Input {
        name: "Rust".into(),
        num: 10,
    };
    let serialized_input = serde_json::to_string(&input)?;

    let stdin = ReadPipe::from(serialized_input);
    let stdout = WritePipe::new_in_memory();

    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(stdin.clone()))
        .stdout(Box::new(stdout.clone()))
        .build();
    let module = Module::from_file(&engine, path)?;

    let mut store = Store::new(&engine, wasi);

    linker
        .module(&mut store, "", &module)
        .expect("linking the function");
    linker
        .get(&mut store, "", "greet")
        .expect("should get the wasi runtime")
        .into_func()
        .expect("Couldnt'turn into func")
        .typed::<(), (), _>(&store)
        .expect("should type the function")
        .call(&mut store, ())
        .expect("should call the function");

    drop(store);

    let contents: Vec<u8> = stdout
        .try_into_inner()
        .map_err(|_err| anyhow::Error::msg("sole remaining reference"))?
        .into_inner();
    let output: Output = serde_json::from_slice(&contents)?;

    Ok(output)
}
