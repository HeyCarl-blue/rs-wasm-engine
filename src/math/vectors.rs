use wasm_bindgen::prelude::wasm_bindgen;

use std::ops;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}
#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new (x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn length_squared (&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length (&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized (&self) -> Vec2 {
        let l = self.length();
        Vec2 { x: self.x / l, y: self.y / l }
    }

    pub fn angle (&self) -> f32 {
        (self.x / self.length()).acos()
    }

    pub fn angle_with_vec2 (&self, rhs: Vec2) -> f32 {
        let dot = self.x * rhs.x + self.y * rhs.y;
        (dot / (self.length() * rhs.length())).acos()
    }
}

// OPERATORS OVERLOADING //
impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self * rhs.x, y: self * rhs.y }
    }
}
impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2 { x: self.x / rhs, y: self.y / rhs }
    }
}
impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 { x: -self.x, y: -self.y }
    }
}
// Dot Product
impl ops::Mul<Vec2> for Vec2 {
    type Output = f32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
#[wasm_bindgen]
impl Vec3 {
    #[wasm_bindgen(constructor)]
    pub fn new (x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared (&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length (&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn normalized (&self) -> Vec3 {
        let l = self.length();
        Vec3 { x: self.x / l, y: self.y / l, z: self.z / l }
    }
    pub fn angle (&self) -> f32 {
        (self.x / self.length()).acos()
    }
    pub fn angle_with_vec3 (&self, rhs: Vec3) -> f32 {
        let dot = self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
        (dot / (self.length() * rhs.length())).acos()
    }
    pub fn cross (&self, rhs: Vec3) -> Vec3 {
        Vec3 { 
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

// OPERATORS OVERLOADING //
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}
// Dot Product
impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
#[wasm_bindgen]
impl Vec4 {
    #[wasm_bindgen(constructor)]
    pub fn new (x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub fn length_squared (&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length (&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized (&self) -> Vec4 {
        let l = self.length();
        Vec4 { x: self.x / l, y: self.y / l, z: self.z / l, w: self.w / l }
    }

    pub fn angle (&self) -> f32 {
        (self.x / self.length()).acos()
    }

    pub fn angle_with_vec4 (&self, rhs: Vec4) -> f32 {
        let dot = self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w;
        (dot / (self.length() * rhs.length())).acos()
    }
}

// OPERATORS OVERLOADING //
impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}
impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}
impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}
impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}
impl ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        Vec4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}
impl ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Vec4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}
// Dot Product
impl ops::Mul<Vec4> for Vec4 {
    type Output = f32;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

// CONSTANT FUNCTIONS //
#[wasm_bindgen]
impl Vec2 {
    pub fn zero () -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }
    pub fn one () -> Vec2 {
        Vec2 { x: 1.0, y: 1.0 }
    }
    pub fn right () -> Vec2 {
        Vec2 { x: 1.0, y: 0.0 }
    }
    pub fn left () -> Vec2 {
        Vec2 { x: -1.0, y: 0.0 }
    }
    pub fn up () -> Vec2 {
        Vec2 { x: 0.0, y: 1.0 }
    }
    pub fn down () -> Vec2 {
        Vec2 { x: 0.0, y: -1.0 }
    }
}
#[wasm_bindgen]
impl Vec3 {
    pub fn zero () -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn one () -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }
    pub fn right () -> Vec3 {
        Vec3 { x: 1.0, y: 0.0, z: 0.0 }
    }
    pub fn left () -> Vec3 {
        Vec3 { x: -1.0, y: 0.0, z: 0.0 }
    }
    pub fn up () -> Vec3 {
        Vec3 { x: 0.0, y: 1.0, z: 0.0 }
    }
    pub fn down () -> Vec3 {
        Vec3 { x: 0.0, y: -1.0, z: 0.0 }
    }
    pub fn forward () -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 1.0 }
    }
    pub fn backward () -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: -1.0 }
    }
}
#[wasm_bindgen]
impl Vec4 {
    pub fn zero () -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    pub fn one () -> Vec4 {
        Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 0.0 }
    }
    pub fn right () -> Vec4 {
        Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    pub fn left () -> Vec4 {
        Vec4 { x: -1.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    pub fn up () -> Vec4 {
        Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 }
    }
    pub fn down () -> Vec4 {
        Vec4 { x: 0.0, y: -1.0, z: 0.0, w: 0.0 }
    }
    pub fn forward () -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 }
    }
    pub fn backward () -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: -1.0, w: 0.0 }
    }
}