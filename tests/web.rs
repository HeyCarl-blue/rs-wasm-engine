#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn vec_angle () {
    use crate::math::vectors::Vec2;

    assert_eq!(Vec2::new(1.0, 0.0).angle(), 0.0);
}