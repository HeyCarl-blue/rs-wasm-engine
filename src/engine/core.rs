use std::cell::RefMut;
use std::rc::Rc;
use std::cell::RefCell;

use glam::Vec3;
use js_sys;
use ruwr_ecs::Entity;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use ruwr_ecs::{Scheduler, World};

use crate::engine::components::ActiveCameraTag;
use crate::engine::components::Camera2DComponent;
use crate::engine::components::Camera3DComponent;
use crate::engine::components::ColliderComponent;
use crate::engine::components::ColliderShapeComponent;
use crate::engine::components::DirectionalLightComponent;
use crate::engine::components::TransformComponent;
use crate::engine::objects::{CollisionResult, SceneObject};
use crate::engine::resources::AmbientLight;
use crate::engine::resources::RenderOptions;
use crate::engine::resources::CollisionCallbacks;
use crate::engine::resources::CollisionEvents;
use crate::engine::resources::MaterialStore;
use crate::engine::resources::MeshStore;
use crate::engine::resources::DeltaTime;
use crate::engine::resources::ShaderStore;
use crate::engine::resources::Viewport;
use crate::engine::systems::CollisionSystem;
use crate::engine::systems::GravitySystem;
use crate::engine::systems::RenderSystem;
use crate::engine::types::ColorRGB;
use crate::engine::types::ColorRGBA;
use crate::engine::types::Vector3;

// ================================================================== //
// ============================= ENGINE ============================= //
// ================================================================== //
#[wasm_bindgen]
pub struct Engine {
    context: WebGl2RenderingContext,
    world: Rc<RefCell<World>>,
    sim_scheduler: Scheduler,    // only runs when playing
    render_scheduler: Scheduler, // always runs
    last_timestamp: f64,
    playing: bool
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Engine {
        let document = web_sys::window()
            .expect("no global window")
            .document()
            .expect("no document on window");

        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap_or_else(|| panic!("canvas #{canvas_id} not found"))
            .dyn_into::<HtmlCanvasElement>()
            .expect("element is not a canvas");

        let context = canvas
            .get_context("webgl2")
            .expect("get_context failed")
            .expect("webgl2 not supported")
            .dyn_into::<WebGl2RenderingContext>()
            .expect("context cast failed");

        context.enable(WebGl2RenderingContext::DEPTH_TEST);

        let mut world = World::new();
        // REGISTER RESOURCES
        world.insert_resource(Viewport::new(canvas.width(), canvas.height()));
        world.insert_resource(MeshStore::new());
        world.insert_resource(MaterialStore::new());
        world.insert_resource(CollisionCallbacks::new());
        world.insert_resource(RenderOptions { debug: false });

        let mut shaders = ShaderStore::new();
        shaders.load_defaults(&context);
        // let lambertian_shader = shaders.load_defaults(&context)
        //     .expect("Couldn't load default shaders");
        world.insert_resource(shaders);

        let world = Rc::new(RefCell::new(world));

        let mut sim_scheduler = Scheduler::new();
        sim_scheduler.add_system(CollisionSystem {});
        sim_scheduler.add_system(GravitySystem {});

        let mut render_scheduler = Scheduler::new();
        render_scheduler.add_system(RenderSystem::new(context.clone()));

        Engine {
            context,
            world,
            sim_scheduler,
            render_scheduler,
            last_timestamp: 0.0,
            playing: true
        }
    }

    pub fn scene(&self) -> Scene {
        Scene {
            world: Rc::clone(&self.world),
            context: self.context.clone()
        }
    }

    #[wasm_bindgen(js_name = "nextFrame")]
    pub fn next_frame(&mut self) {
        let timestamp = web_sys::window()
            .unwrap()
            .performance()
            .unwrap()
            .now();
        let dt = if self.last_timestamp == 0.0 {
            0.0
        } else {
            ((timestamp - self.last_timestamp) / 1000.0) as f32
        };
        self.last_timestamp = timestamp;

        let fire_list: Vec<(js_sys::Function, u32, bool)> = {
            let mut world = self.world.borrow_mut();
            world.insert_resource(DeltaTime(dt));
            if self.playing { self.sim_scheduler.run(&mut world); }
            self.render_scheduler.run(&mut world);

            if self.playing {
                match (world.get_resource::<CollisionEvents>(), world.get_resource::<CollisionCallbacks>()) {
                    (Some(events), Some(callbacks)) => callbacks.collect_fires(events),
                    _ => vec![],
                }
            } else { vec![] }
        };

        for (cb, other_id, is_trigger) in fire_list {
            let result = CollisionResult::new(
                Rc::clone(&self.world),
                self.context.clone(),
                Entity::from_id(other_id),
                is_trigger,
            );
            if let Err(e) = cb.call1(&JsValue::UNDEFINED, &result.into()) {
                crate::console_warn!("onCollision callback threw: {:?}", e);
            }
        }
    }

    #[wasm_bindgen(js_name = "isPlaying")]
    pub fn is_playing (&self) -> bool { self.playing }

    #[wasm_bindgen(js_name = "togglePlaying")]
    pub fn toggle_playing (&mut self) { self.playing = !self.playing; }

    pub fn play (&mut self) { self.playing = true; }

    pub fn stop (&mut self) { self.playing = false; }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);
        let mut world = self.world.borrow_mut();
        if let Some(vp) = world.get_resource_mut::<Viewport>() {
            vp.width = width;
            vp.height = height;
        }
    }

    pub fn clear(&self, color: ColorRGBA) {
        self.context.clear_color(color.r, color.g, color.b, color.a);
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT
        );
    }

    #[wasm_bindgen(js_name = "deltaTime")]
    pub fn delta_time(&self) -> f32 {
        self.world.borrow().get_resource::<DeltaTime>().map(|d| d.0).unwrap_or(0.0)
    }

    #[wasm_bindgen(js_name = "setDebug")]
    pub fn set_debug(&mut self, enabled: bool) {
        if let Some(opts) = self.world.borrow_mut().get_resource_mut::<RenderOptions>() {
            opts.debug = enabled;
        }
    }

    #[wasm_bindgen(js_name = "compileShader")]
    pub fn compile_shader (&mut self, vert_src: &str, frag_src: &str) -> u32 {
        self.world.as_ref().borrow_mut().get_resource_mut::<ShaderStore>().unwrap()
            .compile(&self.context, vert_src, frag_src).expect("Error in shader compilation").0
    }
}

// ================================================================== //
// ============================== SCENE ============================= //
// ================================================================== //
#[wasm_bindgen]
pub struct Scene {
    pub(crate) world: Rc<RefCell<World>>,
    pub(crate) context: WebGl2RenderingContext
}

#[wasm_bindgen]
impl Scene {
    // Lighting
    #[wasm_bindgen(js_name = "setAmbientLight")]
    pub fn set_ambient_light(&self, color: ColorRGB) {
        self.world.borrow_mut().insert_resource(AmbientLight { color: color.into() });
    }

    #[wasm_bindgen(js_name = "addDirectionalLight")]
    pub fn add_directional_light(&self, position: Vector3, direction: Vector3, color: ColorRGB) -> SceneObject {
        let light_obj = SceneObject::new(&self, false);
        light_obj.attach::<DirectionalLightComponent>(DirectionalLightComponent { direction: direction.into(), color: color.into() });
        light_obj.attach::<TransformComponent>(TransformComponent::new(position.into()));
        light_obj
    }

    // Cameras
    #[wasm_bindgen(js_name = "addCamera3D")]
    pub fn add_camera_3d(&self, position: Vector3, fov_degrees: f32, near: f32, far: f32) -> SceneObject {
        let camera_obj = SceneObject::new(&self, false);
        camera_obj.attach::<Camera3DComponent>(Camera3DComponent::new(fov_degrees, near, far));
        camera_obj.attach::<TransformComponent>(TransformComponent::new(position.into()));
        self.make_camera_active(&camera_obj);
        camera_obj
    }

    #[wasm_bindgen(js_name = "addCamera2D")]
    pub fn add_camera_2d(&self, position: Vector3, zoom: f32, near: f32, far: f32) -> SceneObject {
        let camera_obj = SceneObject::new(&self, false);
        camera_obj.attach::<Camera2DComponent>(Camera2DComponent::new(zoom, near, far));
        camera_obj.attach::<TransformComponent>(TransformComponent::new(position.into()));
        self.make_camera_active(&camera_obj);
        camera_obj
    }

    #[wasm_bindgen(js_name = "makeCameraActive")]
    pub fn make_camera_active(&self, camera_id: &SceneObject) {
        let mut world = self.world.borrow_mut();
        let active: Vec<Entity> = world.query::<ActiveCameraTag>()
            .map(|(e, _)| e)
            .collect();
        for entity in active {
            world.remove_component::<ActiveCameraTag>(entity);
        }
        world.add_component(Entity::from_id(camera_id.entity.id()), ActiveCameraTag {});
    }
    
    // Assets
    #[wasm_bindgen(js_name = "createBezierMesh")]
    pub fn create_bezier_mesh(
        &self,
        p0: Vector3, p1: Vector3, p2: Vector3, p3: Vector3,
        width: f32,
        segments: u32,
    ) -> u32 {
        let mut world = self.world.borrow_mut();
        world.get_resource_mut::<MeshStore>().unwrap()
            .load_bezier(
                &self.context,
                p0.into(), p1.into(), p2.into(), p3.into(),
                width, segments,
            ).0
    }

    // Entities
    #[wasm_bindgen(js_name = "addSphereCollider")]
    pub fn add_sphere_collider(&self, entity_id: u32, radius: f32, is_trigger: bool) {
        self.world.borrow_mut().add_component(
            Entity::from_id(entity_id),
            ColliderComponent { shape: ColliderShapeComponent::Sphere { radius }, is_trigger },
        );
    }

    #[wasm_bindgen(js_name = "addAabbCollider")]
    pub fn add_aabb_collider(&self, entity_id: u32, half_x: f32, half_y: f32, half_z: f32, is_trigger: bool) {
        self.world.borrow_mut().add_component(
            Entity::from_id(entity_id),
            ColliderComponent {
                shape: ColliderShapeComponent::Aabb { half_extents: Vec3::new(half_x, half_y, half_z) },
                is_trigger,
            },
        );
    }

    // Entity mutation
    pub fn translate(&self, entity_id: u32, delta: Vector3) {
        let mut world = self.world.borrow_mut();
        let entity = Entity::from_id(entity_id);
        if let Some(t) = world.get_component_mut::<TransformComponent>(entity) {
            t.translate(delta.into());
        }
    }

    // Collision queries
    #[wasm_bindgen(js_name = "getCollisionPairs")]
    pub fn get_collision_pairs(&self) -> Vec<u32> {
        self.world.borrow()
            .get_resource::<CollisionEvents>()
            .map(|ce| ce.events.iter()
                .flat_map(|e| [e.entity_a.id(), e.entity_b.id()])
                .collect())
            .unwrap_or_default()
    }
}
impl Scene {
    pub fn world_mut (&self) -> RefMut<'_, World> {
        self.world.as_ref().borrow_mut()
    }
}
