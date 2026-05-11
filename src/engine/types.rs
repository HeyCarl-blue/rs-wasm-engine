use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use glam::{Mat4, Quat, Vec3, Vec4};
use wasm_bindgen::prelude::wasm_bindgen;

// ================================================================== //
// ============================= VECTOR3 ============================ //
// ================================================================== //

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[wasm_bindgen]
impl Vector3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    pub fn zero()    -> Self { Self::new(0.0,  0.0,  0.0) }
    pub fn one()     -> Self { Self::new(1.0,  1.0,  1.0) }
    pub fn up()      -> Self { Self::new(0.0,  1.0,  0.0) }
    pub fn down()    -> Self { Self::new(0.0, -1.0,  0.0) }
    pub fn right()   -> Self { Self::new(1.0,  0.0,  0.0) }
    pub fn left()    -> Self { Self::new(-1.0, 0.0,  0.0) }
    pub fn forward() -> Self { Self::new(0.0,  0.0, -1.0) }
    pub fn back()    -> Self { Self::new(0.0,  0.0,  1.0) }

    pub fn add(&self, other: Vector3) -> Self { *self + other }
    pub fn sub(&self, other: Vector3) -> Self { *self - other }
    pub fn mul(&self, scalar: f32)    -> Self { *self * scalar }
    pub fn div(&self, scalar: f32)    -> Self { *self / scalar }
    pub fn neg(&self)                 -> Self { -*self }

    pub fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f32          { self.length_squared().sqrt() }
    #[wasm_bindgen(js_name = "lengthSquared")]
    pub fn length_squared(&self) -> f32  { self.dot(*self) }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 { *self / len } else { *self }
    }

    pub fn distance(&self, other: Vector3) -> f32 { (*self - other).length() }

    pub fn lerp(&self, other: Vector3, t: f32) -> Self {
        *self + (other - *self) * t
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        format!("({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

impl Add  for Vector3 { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.x+r.x, self.y+r.y, self.z+r.z) } }
impl Sub  for Vector3 { type Output = Self; fn sub(self, r: Self) -> Self { Self::new(self.x-r.x, self.y-r.y, self.z-r.z) } }
impl Mul<f32> for Vector3 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s, self.y*s, self.z*s) } }
impl Div<f32> for Vector3 { type Output = Self; fn div(self, s: f32) -> Self { Self::new(self.x/s, self.y/s, self.z/s) } }
impl Neg  for Vector3 { type Output = Self; fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) } }
impl AddAssign for Vector3 { fn add_assign(&mut self, r: Self) { self.x+=r.x; self.y+=r.y; self.z+=r.z; } }
impl SubAssign for Vector3 { fn sub_assign(&mut self, r: Self) { self.x-=r.x; self.y-=r.y; self.z-=r.z; } }
impl MulAssign<f32> for Vector3 { fn mul_assign(&mut self, s: f32) { self.x*=s; self.y*=s; self.z*=s; } }
impl DivAssign<f32> for Vector3 { fn div_assign(&mut self, s: f32) { self.x/=s; self.y/=s; self.z/=s; } }

impl From<Vector3> for Vec3 {
    fn from(v: Vector3) -> Self { Self::new(v.x, v.y, v.z) }
}
impl From<Vec3> for Vector3 {
    fn from(v: Vec3) -> Self { Self::new(v.x, v.y, v.z) }
}

// ================================================================== //
// ============================= VECTOR4 ============================ //
// ================================================================== //

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[wasm_bindgen]
impl Vector4 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }

    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) }
    pub fn one()  -> Self { Self::new(1.0, 1.0, 1.0, 1.0) }

    pub fn add(&self, other: Vector4) -> Self { *self + other }
    pub fn sub(&self, other: Vector4) -> Self { *self - other }
    pub fn mul(&self, scalar: f32)    -> Self { *self * scalar }
    pub fn div(&self, scalar: f32)    -> Self { *self / scalar }
    pub fn neg(&self)                 -> Self { -*self }

    pub fn dot(&self, other: Vector4) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w
    }

    pub fn length(&self) -> f32         { self.length_squared().sqrt() }
    #[wasm_bindgen(js_name = "lengthSquared")]
    pub fn length_squared(&self) -> f32 { self.dot(*self) }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 { *self / len } else { *self }
    }

    pub fn lerp(&self, other: Vector4, t: f32) -> Self {
        *self + (other - *self) * t
    }

    pub fn xyz(&self) -> Vector3 { Vector3::new(self.x, self.y, self.z) }
}

impl Add  for Vector4 { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.x+r.x, self.y+r.y, self.z+r.z, self.w+r.w) } }
impl Sub  for Vector4 { type Output = Self; fn sub(self, r: Self) -> Self { Self::new(self.x-r.x, self.y-r.y, self.z-r.z, self.w-r.w) } }
impl Mul<f32> for Vector4 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s, self.y*s, self.z*s, self.w*s) } }
impl Div<f32> for Vector4 { type Output = Self; fn div(self, s: f32) -> Self { Self::new(self.x/s, self.y/s, self.z/s, self.w/s) } }
impl Neg  for Vector4 { type Output = Self; fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z, -self.w) } }

impl From<Vector4> for Vec4 {
    fn from(v: Vector4) -> Self { Self::new(v.x, v.y, v.z, v.w) }
}
impl From<Vec4> for Vector4 {
    fn from(v: Vec4) -> Self { Self::new(v.x, v.y, v.z, v.w) }
}

// ================================================================== //
// ============================ COLORS ============================== //
// ================================================================== //

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[wasm_bindgen]
impl ColorRGB {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32) -> Self { Self { r, g, b } }

    #[wasm_bindgen(js_name = "fromHex")]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let expanded;
        let hex = match hex.len() {
            3 => {
                expanded = format!("{0}{0}{1}{1}{2}{2}", &hex[0..1], &hex[1..2], &hex[2..3]);
                expanded.as_str()
            }
            6 => hex,
            8 => &hex[0..6],
            _ => panic!("invalid hex color: #{hex}"),
        };
        let r = u8::from_str_radix(&hex[0..2], 16).expect("invalid hex") as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).expect("invalid hex") as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).expect("invalid hex") as f32 / 255.0;
        Self { r, g, b }
    }
}
impl From<ColorRGB> for Vec3 {
    fn from(c: ColorRGB) -> Self { Self::new(c.r, c.g, c.b) }
}
impl From<Vec3> for ColorRGB {
    fn from(v: Vec3) -> Self { Self::new(v.x, v.y, v.z) }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[wasm_bindgen]
impl ColorRGBA {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }

    #[wasm_bindgen(js_name = "fromHex")]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let expanded;
        let hex = match hex.len() {
            4 => {
                expanded = format!("{0}{0}{1}{1}{2}{2}{3}{3}", &hex[0..1], &hex[1..2], &hex[2..3], &hex[3..4]);
                expanded.as_str()
            }
            6 => { expanded = format!("{hex}ff"); expanded.as_str() }
            8 => hex,
            _ => panic!("invalid hex color: #{hex}"),
        };
        let r = u8::from_str_radix(&hex[0..2], 16).expect("invalid hex") as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).expect("invalid hex") as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).expect("invalid hex") as f32 / 255.0;
        let a = u8::from_str_radix(&hex[6..8], 16).expect("invalid hex") as f32 / 255.0;
        Self { r, g, b, a }
    }
}
impl From<ColorRGBA> for Vec4 {
    fn from(c: ColorRGBA) -> Self { Self::new(c.r, c.g, c.b, c.a) }
}

// ================================================================== //
// =========================== QUATERNION =========================== //
// ================================================================== //

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[wasm_bindgen]
impl Quaternion {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }

    pub fn identity() -> Self { Self::new(0.0, 0.0, 0.0, 1.0) }

    pub fn from_axis_angle(axis: Vector3, angle_rad: f32) -> Self {
        Quat::from_axis_angle(axis.into(), angle_rad).into()
    }

    pub fn from_euler_xyz(x_rad: f32, y_rad: f32, z_rad: f32) -> Self {
        Quat::from_euler(glam::EulerRot::XYZ, x_rad, y_rad, z_rad).into()
    }

    pub fn from_rotation_arc(from: Vector3, to: Vector3) -> Self {
        Quat::from_rotation_arc(from.into(), to.into()).into()
    }

    pub fn multiply(&self, other: Quaternion) -> Self {
        (Quat::from(*self) * Quat::from(other)).into()
    }

    pub fn rotate_vec(&self, v: Vector3) -> Vector3 {
        (Quat::from(*self) * Vec3::from(v)).into()
    }

    pub fn normalize(&self) -> Self {
        Quat::from(*self).normalize().into()
    }

    pub fn conjugate(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn inverse(&self) -> Self {
        Quat::from(*self).inverse().into()
    }

    pub fn slerp(&self, other: Quaternion, t: f32) -> Self {
        Quat::from(*self).slerp(Quat::from(other), t).into()
    }

    pub fn dot(&self, other: Quaternion) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w
    }
}

impl From<Quaternion> for Quat {
    fn from(q: Quaternion) -> Self { Quat::from_xyzw(q.x, q.y, q.z, q.w) }
}
impl From<Quat> for Quaternion {
    fn from(q: Quat) -> Self { Self { x: q.x, y: q.y, z: q.z, w: q.w } }
}

// ================================================================== //
// ============================ MATRIX4 ============================ //
// ================================================================== //

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    inner: Mat4,
}
#[wasm_bindgen]
impl Matrix4 {
    pub fn identity() -> Self { Self { inner: Mat4::IDENTITY } }

    pub fn from_transform(position: Vector3, rotation: Quaternion, scale: Vector3) -> Self {
        Self { inner: Mat4::from_scale_rotation_translation(scale.into(), rotation.into(), position.into()) }
    }

    pub fn perspective(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self { inner: Mat4::perspective_rh_gl(fov_rad, aspect, near, far) }
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self { inner: Mat4::orthographic_rh_gl(left, right, bottom, top, near, far) }
    }

    pub fn multiply(&self, other: Matrix4) -> Self { Self { inner: self.inner * other.inner } }
    pub fn inverse(&self)                  -> Self { Self { inner: self.inner.inverse() } }

    pub fn transform_point(&self, point: Vector3) -> Vector3 {
        self.inner.transform_point3(point.into()).into()
    }

    pub fn transform_vector(&self, vector: Vector3) -> Vector3 {
        self.inner.transform_vector3(vector.into()).into()
    }

    /// Returns the 16 elements in column-major order as a Float32Array,
    /// ready for `gl.uniformMatrix4fv(loc, false, mat.toArray())`.
    pub fn to_array(&self) -> Vec<f32> { self.inner.to_cols_array().to_vec() }
}
impl From<Mat4> for Matrix4 { fn from(m: Mat4) -> Self { Self { inner: m } } }
impl From<Matrix4> for Mat4 { fn from(m: Matrix4) -> Self { m.inner } }
