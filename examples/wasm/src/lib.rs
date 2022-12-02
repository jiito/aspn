#[no_mangle]
pub extern "C" fn greet() {
    println!("Hello, {}!", "Ben");
}
