pub mod engine;

mod utils;

use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert (s: &str);
}

#[wasm_bindgen(start)]
fn start () {
    set_panic_hook();
}
