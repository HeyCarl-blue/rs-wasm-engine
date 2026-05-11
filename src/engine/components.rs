use glam::{Mat4, Quat, Vec3};
use ruwr_macros::component;

use crate::engine::resources::{MaterialId, MeshId};

#[component]
#[derive(Debug, Clone, PartialEq)]
pub struct TransformComponent {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
} impl TransformComponent {
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

    pub fn forward (&self) -> Vec3 { self.rotation * Vec3::NEG_Z }
    pub fn back    (&self) -> Vec3 { self.rotation * Vec3::Z     }
    pub fn up      (&self) -> Vec3 { self.rotation * Vec3::Y     }
    pub fn down    (&self) -> Vec3 { self.rotation * Vec3::NEG_Y }
    pub fn right   (&self) -> Vec3 { self.rotation * Vec3::X     }
    pub fn left    (&self) -> Vec3 { self.rotation * Vec3::NEG_X }
} impl Default for TransformComponent {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

#[component]
#[derive(Debug, Clone)]
pub struct Camera3DComponent {
    pub fov_degrees: f32,
    pub near: f32,
    pub far: f32
} impl Camera3DComponent {
    pub fn new (fov_degrees: f32, near: f32, far: f32) -> Self {
        Self { fov_degrees, near, far }
    }
} impl Default for Camera3DComponent {
    fn default() -> Self {
        Self { fov_degrees: 45.0, near: 0.1, far: 1000.0 }
    }
}

#[component]
#[derive(Debug, Clone)]
pub struct Camera2DComponent {
    pub zoom: f32,
    pub near: f32,
    pub far: f32,
} impl Camera2DComponent {
    pub fn new (zoom: f32, near: f32, far: f32) -> Self {
        Self { zoom, near, far }
    }
} impl Default for Camera2DComponent {
    fn default() -> Self {
        Self { zoom: 1.0, near: -1000.0, far: 1000.0 }
    }
}

// LIGHT
#[component]
#[derive(Debug, Clone)]
pub struct DirectionalLightComponent {
    pub direction: Vec3,
    pub color: Vec3
} impl DirectionalLightComponent {
    pub fn new (direction: Vec3, color: Vec3) -> Self {
        Self { direction, color }
    }
}

// MESH
#[component]
#[derive(Debug, Clone)]
pub struct MeshComponent {
    pub mesh_id: MeshId
} impl MeshComponent {
    pub fn new (mesh_id: MeshId) -> Self {
        Self { mesh_id }
    }
}

// MATERIALS
#[component]
#[derive(Debug, Clone)]
pub struct MaterialComponent {
    pub material_id: MaterialId,
}

// PHYSICS
#[component]
#[derive(Debug, Clone)]
pub struct RigidbodyComponent {
    pub mass: f32,
    pub velocity: Vec3,
    pub gravity_enabled: bool,
} impl RigidbodyComponent {
    pub fn new(mass: f32, gravity_enabled: bool) -> Self {
        Self { mass, velocity: Vec3::ZERO, gravity_enabled }
    }
}

#[derive(Debug, Clone)]
pub enum ColliderShapeComponent {
    Sphere { radius: f32 },
    Aabb   { half_extents: Vec3 },
}

impl ColliderShapeComponent {
    pub fn aabb(&self, pos: Vec3) -> crate::engine::resources::Aabb {
        match self {
            ColliderShapeComponent::Sphere { radius } => crate::engine::resources::Aabb {
                center: pos,
                half_extents: Vec3::splat(*radius),
            },
            ColliderShapeComponent::Aabb { half_extents } => crate::engine::resources::Aabb {
                center: pos,
                half_extents: *half_extents,
            },
        }
    }
}

#[component]
pub struct ColliderComponent {
    pub shape: ColliderShapeComponent,
    pub is_trigger: bool,
}


// TAGS
#[component]
pub struct VisibleTag {}

#[component]
pub struct ActiveCameraTag {}