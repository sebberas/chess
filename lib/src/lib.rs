use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    println!("Hello, world!");
}

#[wasm_bindgen]
pub fn positions() -> i32 {
    return 1;
}
