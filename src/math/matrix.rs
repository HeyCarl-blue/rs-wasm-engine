use wasm_bindgen::prelude::wasm_bindgen;

use crate::math::vectors::Vec4;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Mat4 {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32,
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32
}
#[wasm_bindgen]
impl Mat4 {
    #[wasm_bindgen(constructor)]
    pub fn new (
        m11: f32, m12: f32, m13: f32, m14: f32,
        m21: f32, m22: f32, m23: f32, m24: f32,
        m31: f32, m32: f32, m33: f32, m34: f32,
        m41: f32, m42: f32, m43: f32, m44: f32
    ) -> Mat4 {
        Mat4 {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn from_vec4_rows (v1: Vec4, v2: Vec4, v3: Vec4, v4: Vec4) -> Mat4 {
        Mat4 {
            m11: v1.x, m12: v1.y, m13: v1.z, m14: v1.w,
            m21: v2.x, m22: v2.y, m23: v2.z, m24: v2.w,
            m31: v3.x, m32: v3.y, m33: v3.z, m34: v3.w,
            m41: v4.x, m42: v4.y, m43: v4.z, m44: v4.w
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn from_vec4_cols (v1: Vec4, v2: Vec4, v3: Vec4, v4: Vec4) -> Mat4 {
        Mat4 {
            m11: v1.x, m12: v2.x, m13: v3.x, m14: v4.x,
            m21: v1.y, m22: v2.y, m23: v3.y, m24: v4.y,
            m31: v1.z, m32: v2.z, m33: v3.z, m34: v4.z,
            m41: v1.w, m42: v2.w, m43: v3.w, m44: v4.w
        }
    }
}