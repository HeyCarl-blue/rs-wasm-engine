pub mod core;
pub mod math;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert (s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}

#[wasm_bindgen(start)]
fn start () {
    
}
