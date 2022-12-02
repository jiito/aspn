use anyhow::{Context, Result};
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

pub fn run(path: &str) -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, path)?;
    linker.module(&mut store, "", &module)?;
    let ext = linker
        .get(&mut store, "", "greet")
        .expect("Can't find extern")
        .into_func();

    match ext {
        Some(f) => f.typed::<(), (), _>(&store)?.call(&mut store, ())?,
        None => {}
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::utils::wasm::run;

    #[test]
    fn run_wasm_file() {
        assert!(
            run("/Users/bjar/git/aspn/examples/wasm/target/wasm32-wasi/debug/wasm.wasm").is_ok()
        );
    }
}
