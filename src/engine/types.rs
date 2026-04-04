use glam::{Vec3, Vec4};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[wasm_bindgen]
impl Vector3 {
    #[wasm_bindgen(constructor)]
    pub fn new (x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
}
impl From<Vector3> for Vec3 {
    fn from(value: Vector3) -> Self {
        Self { x: value.x, y: value.y, z: value.z }
    }
}

#[wasm_bindgen]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[wasm_bindgen]
impl ColorRGB {
    #[wasm_bindgen(constructor)]
    pub fn new (r: f32, g: f32, b: f32) -> Self { Self { r, g, b } }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');

        let expanded;
        let hex = match hex.len() {
            3 => {
                expanded = format!(
                    "{0}{0}{1}{1}{2}{2}",
                    &hex[0..1], &hex[1..2], &hex[2..3]
                );
                expanded.as_str()
            }
            6 => hex,
            8 => &hex[0..6],
            _ => panic!("invalid hex color: #{hex}"),
        };

        let r = u8::from_str_radix(&hex[0..2], 16).expect("invalid hex color") as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).expect("invalid hex color") as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).expect("invalid hex color") as f32 / 255.0;

        Self { r, g, b }
    }

}
impl From<ColorRGB> for Vec3 {
    fn from(value: ColorRGB) -> Self {
        Self { x: value.r, y: value.g, z: value.b }
    }
}

#[wasm_bindgen]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[wasm_bindgen]
impl ColorRGBA {
    #[wasm_bindgen(constructor)]
    pub fn new (r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');

        let expanded;
        let hex = match hex.len() {
            4 => {
                expanded = format!(
                    "{0}{0}{1}{1}{2}{2}{3}{3}",
                    &hex[0..1], &hex[1..2], &hex[2..3], &hex[3..4]
                );
                expanded.as_str()
            }
            6 => {
                expanded = format!(
                    "{0}ff",
                    &hex, 
                );
                expanded.as_str()
            },
            8 => hex,
            _ => panic!("invalid hex color: #{hex}"),
        };

        let r = u8::from_str_radix(&hex[0..2], 16).expect("invalid hex color") as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).expect("invalid hex color") as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).expect("invalid hex color") as f32 / 255.0;
        let a = u8::from_str_radix(&hex[6..8], 16).expect("invalid hex color") as f32 / 255.0;

        Self { r, g, b, a }
    }
}
impl From<ColorRGBA> for Vec4 {
    fn from(value: ColorRGBA) -> Self {
        Self { x: value.r, y: value.g, z: value.b, w: value.a }
    }
}