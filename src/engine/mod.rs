pub mod components;
pub mod systems;
pub mod resources;
pub mod types;

use glam::Vec3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use ruwr_ecs::{Scheduler, World};

use components::Viewport;

use crate::engine::components::ActiveCamera;
use crate::engine::components::Camera3D;
use crate::engine::components::DirectionalLight;
use crate::engine::components::Material;
use crate::engine::components::Mesh;
use crate::engine::components::Transform;
use crate::engine::components::Visible;
use crate::engine::resources::AmbientLight;
use crate::engine::resources::MaterialData;
use crate::engine::resources::MaterialId;
use crate::engine::resources::MaterialStore;
use crate::engine::resources::MeshId;
use crate::engine::resources::ShaderId;
use crate::engine::resources::{DeltaTime, MeshStore, ShaderStore};
use crate::engine::systems::RenderSystem;
use crate::engine::types::ColorRGB;
use crate::engine::types::ColorRGBA;
use crate::engine::types::Vector3;

#[wasm_bindgen]
pub struct Engine {
    context: WebGl2RenderingContext,
    world: World,
    scheduler: Scheduler,
    last_timestamp: f64,
    lambertian_shader: ShaderId,
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
        world.insert_resource(Viewport::new(canvas.width(), canvas.height()));
        world.insert_resource(MeshStore::new());
        world.insert_resource(MaterialStore::new());

        let camera = world.spawn();
        world.add_component(camera, Camera3D::new(45.0, 0.1, 100.0));
        world.add_component(camera, Transform::new(Vec3::new(0.0, 0.0, 0.0)));
        world.add_component(camera, ActiveCamera {});

        let mut shaders = ShaderStore::new();
        let lambertian_shader = shaders.load_defaults(&context)
            .expect("Couldn't load default shaders");
        world.insert_resource(shaders);

        let mut scheduler = Scheduler::new();
        scheduler.add_system(RenderSystem::new(context.clone()));

        Engine {
            context,
            world,
            scheduler,
            last_timestamp: 0.0,
            lambertian_shader,
        }
    }

    pub fn next_frame(&mut self, timestamp: f64) {
        let dt = if self.last_timestamp == 0.0 {
            0.0
        } else {
            ((timestamp - self.last_timestamp) / 1000.0) as f32
        };

        self.last_timestamp = timestamp;
        self.world.insert_resource(DeltaTime(dt));
        self.scheduler.run(&mut self.world);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);
        if let Some(vp) = self.world.get_resource_mut::<Viewport>() {
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
}

// Scene API

#[wasm_bindgen]
impl Engine {
    pub fn set_ambient_light(&mut self, color: ColorRGB) {
        self.world.insert_resource(AmbientLight { color: color.into() });
    }

    pub fn add_directional_light(&mut self, direction: Vector3, color: ColorRGB) -> u32 {
        let light = self.world.spawn();
        let direction: Vec3 = direction.into();
        self.world.add_component(light, DirectionalLight {
            direction: direction.normalize(),
            color: color.into(),
        });
        light.id()
    }

    // Asset creation

    pub fn create_sphere_mesh(&mut self, stacks: u32, slices: u32) -> u32 {
        self.world.get_resource_mut::<MeshStore>().unwrap()
            .get_or_create_sphere(&self.context, stacks, slices).0
    }

    pub fn create_lambertian_material(&mut self, albedo: ColorRGB) -> u32 {
        self.world.get_resource_mut::<MaterialStore>().unwrap()
            .insert(MaterialData { shader_id: self.lambertian_shader, albedo: albedo.into() }).0
    }

    // Entity spawning

    pub fn spawn_object(&mut self, mesh_id: u32, material_id: u32, position: Vector3) -> u32 {
        let entity = self.world.spawn();
        self.world.add_component(entity, Transform::new(position.into()));
        self.world.add_component(entity, Mesh::new(MeshId(mesh_id), 0));
        self.world.add_component(entity, Material { material_id: MaterialId(material_id) });
        self.world.add_component(entity, Visible {});
        entity.id()
    }

    // Helpers

    pub fn add_lambertian_sphere(&mut self, position: Vector3, albedo: ColorRGB, stacks: u32, slices: u32) -> u32 {
        let mesh_id = self.create_sphere_mesh(stacks, slices);
        let mat_id  = self.create_lambertian_material(albedo);
        self.spawn_object(mesh_id, mat_id, position)
    }
}
