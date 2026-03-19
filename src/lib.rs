pub mod core;
pub mod math;

mod utils;

use std::panic;

use wasm_bindgen::prelude::*;

use crate::utils::set_panick_hook;

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
    set_panick_hook();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_angle () {
        assert_eq!(math::vectors::Vec2::new(f32::cos(0.25), f32::sin(0.25)).angle(), 0.25);
    }
}
