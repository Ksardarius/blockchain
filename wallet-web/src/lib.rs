use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello, {} from Rust WebAssembly!", name).into());
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    console::log_1(&format!("Adding {} and {} in Rust Wasm...", a, b).into());
    a + b
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console::log_1(&"Rust WebAssembly module loaded!".into());
    Ok(())
}
