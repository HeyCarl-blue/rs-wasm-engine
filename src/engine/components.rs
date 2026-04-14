use glam::{Mat4, Quat, Vec3};
use ruwr_macros::component;

use crate::engine::resources::{MaterialId, MeshId};

#[component]
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
} impl Transform {
    pub fn new (position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE
        }
    }

    pub fn translate (&mut self, delta: Vec3) {
        self.position.x += delta.x;
        self.position.y += delta.y;
        self.position.z += delta.z;
    }

    pub fn look_at (&mut self, target: Vec3) {
        let dir = (target - self.position).normalize();
        self.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
    }

    pub fn matrix (&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
} impl Default for Transform {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

#[component]
#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: u32,
    pub height: u32
} impl Viewport {
    pub fn new (width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio (&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn top (&self) -> f32 {
        self.height as f32 / 2.0
    }

    pub fn bottom (&self) -> f32 {
        self.height as f32 / -2.0
    }

    pub fn right (&self) -> f32 {
        self.width as f32 / 2.0
    }

    pub fn left (&self) -> f32 {
        self.width as f32 / -2.0
    }
}

// CAMERA

#[component]
pub struct ActiveCamera {}

#[component]
#[derive(Debug, Clone)]
pub struct Camera3D {
    pub fov_degrees: f32,
    pub near: f32,
    pub far: f32
} impl Camera3D {
    pub fn new (fov_degrees: f32, near: f32, far: f32) -> Self {
        Self { fov_degrees, near, far }
    }
} impl Default for Camera3D {
    fn default() -> Self {
        Self { fov_degrees: 45.0, near: 0.1, far: 1000.0 }
    }
}

#[component]
#[derive(Debug, Clone)]
pub struct Camera2D {
    pub zoom: f32,
    pub near: f32,
    pub far: f32,
} impl Camera2D {
    pub fn new (zoom: f32) -> Self {
        Self { zoom, near: -1.0, far: 1.0 }
    }
} impl Default for Camera2D {
    fn default() -> Self {
        Self { zoom: 1.0, near: -1000.0, far: 1000.0 }
    }
}

#[component]
pub struct Visible {}

// LIGHT
#[component]
#[derive(Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3
} impl DirectionalLight {
    pub fn new (direction: Vec3, color: Vec3) -> Self {
        Self { direction, color }
    }
}

// MESH
#[component]
#[derive(Debug, Clone)]
pub struct Mesh {
    pub mesh_id: MeshId
} impl Mesh {
    pub fn new (mesh_id: MeshId) -> Self {
        Self { mesh_id }
    }
}

// MATERIALS
#[component]
#[derive(Debug, Clone)]
pub struct Material {
    pub material_id: MaterialId,
}
