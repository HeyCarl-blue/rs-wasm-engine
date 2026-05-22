use std::cell::RefCell;
use std::rc::Rc;

use glam::{Quat, Vec3};
use ruwr_ecs::{Component, Entity, World};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGl2RenderingContext;

use crate::{console_warn, engine::{components::{Camera2DComponent, Camera3DComponent, ColliderComponent, ColliderShapeComponent, DirectionalLightComponent, MaterialComponent, MeshComponent, RigidbodyComponent, SphereParamsComponent, TransformComponent, VisibleTag}, core::Scene, resources::{CollisionCallbacks, MaterialData, MaterialStore, MeshStore, ShaderStore}, types::{ColorRGB, Matrix4, Quaternion, Vector3}}};

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

    pub fn id (&self) -> u32 { self.entity.id() }

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

    #[wasm_bindgen(js_name = "onCollision")]
    pub fn on_collision(&self, cb: js_sys::Function) {
        self.world.borrow_mut()
            .get_resource_mut::<CollisionCallbacks>()
            .expect("CollisionCallbacks resource missing")
            .insert_callback(self.entity.id(), cb);
    }

    #[wasm_bindgen(js_name = "getTransform")]
    pub fn get_transform (&self) -> Option<Transform> {
        if let Some(transform) = self.world.borrow().get_component::<TransformComponent>(self.entity) {
            Some(Transform { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), position: transform.position, rotation: transform.rotation, scale: transform.scale })
        } else {
            console_warn!("tried to get Transform from an object that has no Transform attached");
            None
        }
    }

    #[wasm_bindgen(js_name = "getDirectionalLight")]
    pub fn get_directional_light(&self) -> Option<DirectionalLight> {
        let world = self.world.borrow();
        if let Some(c) = world.get_component::<DirectionalLightComponent>(self.entity) {
            Some(DirectionalLight { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), direction: c.direction.into(), color: c.color.into() })
        } else {
            console_warn!("tried to get DirectionalLight from an object that has no DirectionalLight attached");
            None
        }
    }

    #[wasm_bindgen(js_name = "getCamera2D")]
    pub fn get_camera_2d(&self) -> Option<Camera2D> {
        let world = self.world.borrow();
        if let Some(c) = world.get_component::<Camera2DComponent>(self.entity) {
            Some(Camera2D { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), zoom: c.zoom, near: c.near, far: c.far })
        } else {
            console_warn!("tried to get Camera2D from an object that has no Camera2D attached");
            None
        }
    }

    #[wasm_bindgen(js_name = "getCamera3D")]
    pub fn get_camera_3d(&self) -> Option<Camera3D> {
        let world = self.world.borrow();
        if let Some(c) = world.get_component::<Camera3DComponent>(self.entity) {
            Some(Camera3D { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), fov_degrees: c.fov_degrees, near: c.near, far: c.far })
        } else {
            console_warn!("tried to get Camera3D from an object that has no Camera3D attached");
            None
        }
    }

    #[wasm_bindgen(js_name = "getRigidbody")]
    pub fn get_rigidbody(&self) -> Option<Rigidbody> {
        let world = self.world.borrow();
        if let Some(c) = world.get_component::<RigidbodyComponent>(self.entity) {
            Some(Rigidbody { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), mass: c.mass, gravity_enabled: c.gravity_enabled })
        } else {
            console_warn!("tried to get Rigidbody from an object that has no Rigidbody attached");
            None
        }
    }

    #[wasm_bindgen(js_name = "getLambertianMaterial")]
    pub fn get_lambertian_material(&self) -> Option<LambertianMaterial> {
        let world = self.world.borrow();
        let mat_id = world.get_component::<MaterialComponent>(self.entity).map(|c| c.material_id);
        if let Some(mat_id) = mat_id {
            let albedo = world.get_resource::<MaterialStore>().and_then(|s| s.get(mat_id)).map(|d| d.albedo.into());
            if let Some(albedo) = albedo {
                return Some(LambertianMaterial { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), albedo });
            }
        }
        console_warn!("tried to get LambertianMaterial from an object that has no LambertianMaterial attached");
        None
    }

    #[wasm_bindgen(js_name = "getSphereMesh")]
    pub fn get_sphere_mesh(&self) -> Option<SphereMesh> {
        let world = self.world.borrow();
        if let Some(p) = world.get_component::<SphereParamsComponent>(self.entity) {
            Some(SphereMesh { world: Some(Rc::clone(&self.world)), entity: Some(self.entity), context: Some(self.context.clone()), stacks: p.stacks, slices: p.slices })
        } else {
            console_warn!("tried to get SphereMesh from an object that has no SphereMesh attached");
            None
        }
    }

    #[wasm_bindgen(js_name = "getSphereCollider")]
    pub fn get_sphere_collider(&self) -> Option<SphereCollider> {
        let world = self.world.borrow();
        if let Some(c) = world.get_component::<ColliderComponent>(self.entity) {
            match c.shape {
                ColliderShapeComponent::Sphere { radius } => Some(SphereCollider {
                    world: Some(Rc::clone(&self.world)), entity: Some(self.entity), is_trigger: c.is_trigger, radius,
                }),
                _ => {
                    console_warn!("tried to get SphereCollider but the attached collider is not a sphere");
                    None
                }
            }
        } else {
            console_warn!("tried to get SphereCollider from an object that has no collider attached");
            None
        }
    }
}
impl SceneObject {
    pub(crate) fn from_parts(world: Rc<RefCell<World>>, context: WebGl2RenderingContext, entity: Entity) -> Self {
        Self { world, context, entity }
    }

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
pub struct DirectionalLight {
    world: Option<Rc<RefCell<World>>>,
    entity: Option<Entity>,
    direction: Vector3,
    color:     ColorRGB
}
#[wasm_bindgen]
impl DirectionalLight {
    #[wasm_bindgen(constructor)]
    pub fn new (direction: Vector3, color: ColorRGB) -> Self {
        Self { world: None, entity: None, direction, color }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to(&mut self, obj: &SceneObject) {
        obj.attach::<DirectionalLightComponent>(DirectionalLightComponent { direction: self.direction.into(), color: self.color.into() });
        self.world = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
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
        obj.attach::<SphereParamsComponent>(SphereParamsComponent { stacks: self.stacks, slices: self.slices });
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

// ===========================================================================================
//====================================== COLLISIONS ==========================================
// ===========================================================================================
#[wasm_bindgen]
pub struct SphereCollider {
    world:   Option<Rc<RefCell<World>>>,
    entity:  Option<Entity>,
    is_trigger: bool,
    radius: f32,
}
#[wasm_bindgen]
impl SphereCollider {
    #[wasm_bindgen(constructor)]
    pub fn new (#[wasm_bindgen(js_name = "isTrigger")]is_trigger: bool, radius: f32) -> Self {
        Self { world: None, entity: None, is_trigger, radius }
    }

    #[wasm_bindgen(js_name = "attachTo")]
    pub fn attach_to (&mut self, obj: &SceneObject) {
        obj.attach::<ColliderComponent>(ColliderComponent { shape: ColliderShapeComponent::Sphere { radius: self.radius }, is_trigger: self.is_trigger });
        self.world  = Some(Rc::clone(&obj.world));
        self.entity = Some(obj.entity);
    }

    #[wasm_bindgen(js_name = "onCollision")]
    pub fn on_collision (&mut self, function: js_sys::Function) {
        if let Some(world) = &self.world {
            world.borrow_mut().get_resource_mut::<CollisionCallbacks>().unwrap().insert_callback(self.entity.unwrap().id(), function);
        } else {
            crate::console_warn!("onCollision called before being attached to a SceneObject - callback ignored");
        }
    }
}

#[wasm_bindgen]
pub struct CollisionResult {
    world:        Rc<RefCell<World>>,
    context:      WebGl2RenderingContext,
    other_entity: Entity,
    is_trigger:   bool,
}
#[wasm_bindgen]
impl CollisionResult {
    #[wasm_bindgen(js_name = "getOther")]
    pub fn get_other(&self) -> SceneObject {
        SceneObject::from_parts(Rc::clone(&self.world), self.context.clone(), self.other_entity)
    }

    #[wasm_bindgen(js_name = "isTrigger")]
    pub fn is_trigger(&self) -> bool { self.is_trigger }

    #[wasm_bindgen(js_name = "getOtherId")]
    pub fn get_other_id(&self) -> u32 { self.other_entity.id() }
}
impl CollisionResult {
    pub(crate) fn new(world: Rc<RefCell<World>>, context: WebGl2RenderingContext, other_entity: Entity, is_trigger: bool) -> Self {
        Self { world, context, other_entity, is_trigger }
    }
}
