use std::cell::RefCell;
use std::rc::Rc;

use glam::{Quat, Vec3};
use ruwr_ecs::{Component, Entity, World};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGl2RenderingContext;

use crate::engine::{components::{Camera2DComponent, Camera3DComponent, MaterialComponent, MeshComponent, RigidbodyComponent, TransformComponent, VisibleTag}, core::Scene, resources::{MaterialData, MaterialStore, MeshStore, ShaderStore}, types::{ColorRGB, Matrix4, Quaternion, Vector3}};

#[wasm_bindgen]
pub enum MaterialType {
    LAMBERTIAN
}

#[wasm_bindgen]
pub struct SceneObject {
    world: Rc<RefCell<World>>,
    pub(crate) context: WebGl2RenderingContext,
    pub(crate) entity: Entity
}
#[wasm_bindgen]
impl SceneObject {
    #[wasm_bindgen(constructor)]
    pub fn new (scene: &Scene, visible: bool) -> Self {
        let entity = scene.world.borrow_mut().spawn();
        let obj = Self {
            world: Rc::clone(&scene.world),
            context: scene.context.clone(),
            entity,
        };

        if visible {
            obj.make_visible();
        }

        obj
    }

    #[wasm_bindgen(js_name = "getTransform")]
    pub fn get_transform (&self) -> Option<Transform> {
        if let Some(transform) = self.world.borrow().get_component::<TransformComponent>(self.entity) {
            Some(Transform { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), position: transform.position, rotation: transform.rotation, scale: transform.scale })
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = "makeVisible")]
    pub fn make_visible (&self) {
        self.world.borrow_mut().add_component::<VisibleTag>(self.entity, VisibleTag {});
    }

    #[wasm_bindgen(js_name = "makeInvisible")]
    pub fn make_invisible (&self) {
        self.world.borrow_mut().remove_component::<VisibleTag>(self.entity);
    }

    #[wasm_bindgen(js_name = "isVisible")]
    pub fn is_visible (&self) -> bool {
        self.world.borrow().has_component::<VisibleTag>(self.entity)
    }
}
impl SceneObject {
    pub(crate) fn attach<T: Component>(&self, component: T) {
        self.world.borrow_mut().add_component::<T>(self.entity, component);
    }
}

// =================================================================
// ==================== MODULES (COMPONENTS) =======================
// =================================================================
#[wasm_bindgen]
pub struct Transform {
    world: Option<Rc<RefCell<World>>>,
    entity: Option<Entity>,
    position: Vec3,
    rotation: Quat,
    scale:    Vec3,
}
#[wasm_bindgen]
impl Transform {
    #[wasm_bindgen(constructor)]
    pub fn new (position: Vector3) -> Self {
        Self { world: None, entity: None, position: position.into(), rotation: Quat::IDENTITY, scale: Vec3::ONE }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        obj.attach::<TransformComponent>(TransformComponent { position: self.position, rotation: self.rotation, scale: self.scale });
        self.world = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
    }

    #[wasm_bindgen(js_name = "getPosition")]
    pub fn get_position(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.position.into())
    }

    #[wasm_bindgen(js_name = "setPosition")]
    pub fn set_position(&self, position: Vector3) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<TransformComponent>(e).map(|t| t.position = position.into());
        }
    }

    #[wasm_bindgen(js_name = "getRotation")]
    pub fn get_rotation(&self) -> Option<Quaternion> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.rotation.into())
    }

    #[wasm_bindgen(js_name = "setRotation")]
    pub fn set_rotation(&self, rotation: Quaternion) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<TransformComponent>(e).map(|t| t.rotation = rotation.into());
        }
    }

    #[wasm_bindgen(js_name = "getScale")]
    pub fn get_scale(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.scale.into())
    }

    #[wasm_bindgen(js_name = "setScale")]
    pub fn set_scale(&self, scale: Vector3) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<TransformComponent>(e).map(|t| t.scale = scale.into());
        }
    }

    pub fn translate(&self, delta: Vector3) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<TransformComponent>(e).map(|t| t.translate(delta.into()));
        }
    }

    #[wasm_bindgen(js_name = "lookAt")]
    pub fn look_at(&self, target: Vector3) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            if let Some(t) = w.borrow_mut().get_component_mut::<TransformComponent>(e) {
                let dir = (Vec3::from(target) - t.position).normalize();
                t.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
            }
        }
    }

    pub fn matrix(&self) -> Option<Matrix4> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.matrix().into())
    }

    pub fn forward(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.forward().into())
    }
    pub fn back(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.back().into())
    }
    pub fn up(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.up().into())
    }
    pub fn down(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.down().into())
    }
    pub fn right(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.right().into())
    }
    pub fn left(&self) -> Option<Vector3> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<TransformComponent>(e).map(|t| t.left().into())
    }
}

#[wasm_bindgen]
pub struct Camera2D {
    world: Option<Rc<RefCell<World>>>,
    entity: Option<Entity>,
    zoom: f32,
    near: f32,
    far:  f32,
}
#[wasm_bindgen]
impl Camera2D {
    #[wasm_bindgen(constructor)]
    pub fn new(zoom: f32, near: f32, far: f32) -> Self {
        Self { world: None, entity: None, zoom, near, far }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        obj.attach::<Camera2DComponent>(Camera2DComponent::new(self.zoom, self.near, self.far));
        self.world  = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
    }

    #[wasm_bindgen(js_name = "getZoom")]
    pub fn get_zoom(&self) -> Option<f32> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<Camera2DComponent>(e).map(|c| c.zoom)
    }
    #[wasm_bindgen(js_name = "setZoom")]
    pub fn set_zoom(&self, zoom: f32) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<Camera2DComponent>(e).map(|c| c.zoom = zoom);
        }
    }

    #[wasm_bindgen(js_name = "getNear")]
    pub fn get_near(&self) -> Option<f32> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<Camera2DComponent>(e).map(|c| c.near)
    }
    #[wasm_bindgen(js_name = "setNear")]
    pub fn set_near(&self, near: f32) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<Camera2DComponent>(e).map(|c| c.near = near);
        }
    }

    #[wasm_bindgen(js_name = "getFar")]
    pub fn get_far(&self) -> Option<f32> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<Camera2DComponent>(e).map(|c| c.far)
    }
    #[wasm_bindgen(js_name = "setFar")]
    pub fn set_far(&self, far: f32) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<Camera2DComponent>(e).map(|c| c.far = far);
        }
    }
}

#[wasm_bindgen]
pub struct Camera3D {
    world: Option<Rc<RefCell<World>>>,
    entity: Option<Entity>,
    fov_degrees: f32,
    near: f32,
    far:  f32,
}
#[wasm_bindgen]
impl Camera3D {
    #[wasm_bindgen(constructor)]
    pub fn new(fov_degrees: f32, near: f32, far: f32) -> Self {
        Self { world: None, entity: None, fov_degrees, near, far }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        obj.attach::<Camera3DComponent>(Camera3DComponent::new(self.fov_degrees, self.near, self.far));
        self.world  = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
    }

    #[wasm_bindgen(js_name = "getFovDegrees")]
    pub fn get_fov_degrees(&self) -> Option<f32> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<Camera3DComponent>(e).map(|c| c.fov_degrees)
    }
    #[wasm_bindgen(js_name = "setFovDegrees")]
    pub fn set_fov_degrees(&self, fov_degrees: f32) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<Camera3DComponent>(e).map(|c| c.fov_degrees = fov_degrees);
        }
    }

    #[wasm_bindgen(js_name = "getNear")]
    pub fn get_near(&self) -> Option<f32> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<Camera3DComponent>(e).map(|c| c.near)
    }
    #[wasm_bindgen(js_name = "setNear")]
    pub fn set_near(&self, near: f32) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<Camera3DComponent>(e).map(|c| c.near = near);
        }
    }

    #[wasm_bindgen(js_name = "getFar")]
    pub fn get_far(&self) -> Option<f32> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        w.borrow().get_component::<Camera3DComponent>(e).map(|c| c.far)
    }
    #[wasm_bindgen(js_name = "setFar")]
    pub fn set_far(&self, far: f32) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            w.borrow_mut().get_component_mut::<Camera3DComponent>(e).map(|c| c.far = far);
        }
    }
}

#[wasm_bindgen]
pub struct Rigidbody {
    world: Option<Rc<RefCell<World>>>,
    entity: Option<Entity>,
    mass: f32,
    gravity_enabled: bool,
}
#[wasm_bindgen]
impl Rigidbody {
    #[wasm_bindgen(constructor)]
    pub fn new (mass: f32, gravity_enabled: bool) -> Self {
        Self { world: None, entity: None, mass, gravity_enabled }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        obj.attach::<RigidbodyComponent>(RigidbodyComponent::new(self.mass, self.gravity_enabled));
        self.world  = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
    }
}

// ===========================================================================================
//======================================== MATERIALS =========================================
// ===========================================================================================
#[wasm_bindgen]
pub struct LambertianMaterial {
    world: Option<Rc<RefCell<World>>>,
    entity: Option<Entity>,
    albedo: ColorRGB,
}
#[wasm_bindgen]
impl LambertianMaterial {
    #[wasm_bindgen(constructor)]
    pub fn new(albedo: ColorRGB) -> Self {
        Self { world: None, entity: None, albedo }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        let lambertian_id = obj.world.borrow_mut().get_resource_mut::<ShaderStore>().unwrap()
            .lambertian_id().expect("No lambertian shader compiled");
        let mat_id = obj.world.borrow_mut().get_resource_mut::<MaterialStore>().unwrap()
            .insert(MaterialData { shader_id: lambertian_id, albedo: self.albedo.into() });
        obj.attach::<MaterialComponent>(MaterialComponent { material_id: mat_id });
        self.world  = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
    }

    #[wasm_bindgen(js_name = "getAlbedo")]
    pub fn get_albedo(&self) -> Option<ColorRGB> {
        let (w, e) = (self.world.as_ref()?, self.entity?);
        let world = w.borrow();
        let mat_id = world.get_component::<MaterialComponent>(e)?.material_id;
        world.get_resource::<MaterialStore>()?.get(mat_id).map(|d| d.albedo.into())
    }

    #[wasm_bindgen(js_name = "setAlbedo")]
    pub fn set_albedo(&self, albedo: ColorRGB) {
        if let (Some(w), Some(e)) = (&self.world, self.entity) {
            let mat_id = w.borrow().get_component::<MaterialComponent>(e).map(|c| c.material_id);
            if let Some(mat_id) = mat_id {
                w.borrow_mut().get_resource_mut::<MaterialStore>()
                    .and_then(|s| s.get_mut(mat_id))
                    .map(|d| d.albedo = albedo.into());
            }
        }
    }
}

// ===========================================================================================
//========================================= MESHES ===========================================
// ===========================================================================================
trait Mesh {
    fn rebuild (&mut self);
}

#[wasm_bindgen]
pub struct SphereMesh {
    world:   Option<Rc<RefCell<World>>>,
    entity:  Option<Entity>,
    context: Option<WebGl2RenderingContext>,
    stacks:  u32,
    slices:  u32,
}
#[wasm_bindgen]
impl SphereMesh {
    #[wasm_bindgen(constructor)]
    pub fn new(stacks: u32, slices: u32) -> Self {
        Self { world: None, entity: None, context: None, stacks, slices }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        let mesh_id = obj.world.borrow_mut().get_resource_mut::<MeshStore>()
            .unwrap().get_or_create_sphere(&obj.context, self.stacks, self.slices);
        obj.attach::<MeshComponent>(MeshComponent { mesh_id });
        self.world   = Some(Rc::clone(&obj.world));
        self.entity  = Some(obj.entity);
        self.context = Some(obj.context.clone());
    }

    #[wasm_bindgen(js_name = "getStacks")]
    pub fn get_stacks(&self) -> u32 { self.stacks }

    #[wasm_bindgen(js_name = "getSlices")]
    pub fn get_slices(&self) -> u32 { self.slices }

    #[wasm_bindgen(js_name = "setStacks")]
    pub fn set_stacks (&mut self, stacks: u32) {
        self.stacks = stacks;
        self.rebuild();
    }

    #[wasm_bindgen(js_name = "setSlices")]
    pub fn set_slices (&mut self, slices: u32) {
        self.slices = slices;
        self.rebuild();
    }
}
impl Mesh for SphereMesh {
    fn rebuild(&mut self) {
        if let (Some(w), Some(e), Some(ctx)) = (&self.world, self.entity, &self.context) {
            let mesh_id = w.borrow_mut().get_resource_mut::<MeshStore>()
                .unwrap().get_or_create_sphere(ctx, self.stacks, self.slices);
            w.borrow_mut().get_component_mut::<MeshComponent>(e).map(|m| m.mesh_id = mesh_id);
        }
    }
}
