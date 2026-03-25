pub mod engine;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert (s: &str);
}

#[wasm_bindgen(start)]
fn start () {
    
}
