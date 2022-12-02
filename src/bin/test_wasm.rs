use anyhow::Result;
use aspn::utils::wasm;

fn main() -> Result<()> {
    wasm::run("/Users/bjar/git/aspn/examples/wasm/target/wasm32-wasi/debug/wasm.wasm")
}
