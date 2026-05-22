pub mod engine;

mod utils;

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => {
        web_sys::console::warn_1(&format!($($t)*).into())
    };
}

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
