use glam::{Mat4, Quat, Vec3};
use ruwr_ecs::System;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::engine::{components::{ActiveCameraTag, Camera2DComponent, Camera3DComponent, ColliderComponent, ColliderShapeComponent, DirectionalLightComponent, MaterialComponent, MeshComponent, RigidbodyComponent, TransformComponent, VisibleTag}, resources::{Aabb, CollisionEvent, CollisionEvents, Octree, Viewport}};
use ruwr_ecs::Entity;
use crate::engine::resources::{AmbientLight, DeltaTime, MaterialStore, MeshStore, ShaderStore};

use ruwr_ecs::World;

fn get_view_proj (camera: &Camera3DComponent, transform: &TransformComponent, aspect_ratio: f32) -> Mat4 {
    let proj = Mat4::perspective_rh_gl(
        camera.fov_degrees.to_radians(),
        aspect_ratio,
        camera.near,
        camera.far
    );
    let view = Mat4::look_to_rh(
        transform.position,
        transform.rotation * Vec3::NEG_Z,
        Vec3::Y
    );

    proj * view
}

fn get_ortho_view_proj (camera: &Camera2DComponent, transform: &TransformComponent, viewport: &Viewport) -> Mat4 {
    let hw = (viewport.width as f32 / 2.0) / camera.zoom;
    let hh = (viewport.height as f32 / 2.0) / camera.zoom;

    let proj = Mat4::orthographic_rh_gl(-hw, hw, -hh, hh, camera.near, camera.far);

    let view = Mat4::from_rotation_translation(
        Quat::from_rotation_z(transform.rotation.to_euler(glam::EulerRot::ZYX).0),
        transform.position
    ).inverse();

    proj * view
}

fn set_uniform_mat4(ctx: &WebGl2RenderingContext, prog: &WebGlProgram, name: &str, mat: &Mat4) {
    let loc = ctx.get_uniform_location(prog, name);
    ctx.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, &mat.to_cols_array());
}

fn set_uniform_vec3(ctx: &WebGl2RenderingContext, prog: &WebGlProgram, name: &str, v: Vec3) {
    let loc = ctx.get_uniform_location(prog, name);
    ctx.uniform3fv_with_f32_array(loc.as_ref(), &[v.x, v.y, v.z]);
}

pub struct RenderSystem {
    context: WebGl2RenderingContext
} impl RenderSystem {
    pub fn new (context: WebGl2RenderingContext) -> Self {
        Self { context }
    }
} impl System for RenderSystem {
    fn run(&mut self, world: &mut World) {
        let ctx = &self.context;

        let Some(vp) = world.get_resource::<Viewport>() else { return };
        let aspect_ratio = vp.aspect_ratio();

        let Some(cam_entity) = world.query::<ActiveCameraTag>().next().map(|(e, _)| e) else { return };
        let Some(cam_transform) = world.get_component::<TransformComponent>(cam_entity) else { return };
        let view_proj = if let Some(cam) = world.get_component::<Camera3DComponent>(cam_entity) {
            get_view_proj(cam, cam_transform, aspect_ratio)
        } else if let Some(cam) = world.get_component::<Camera2DComponent>(cam_entity) {
            get_ortho_view_proj(cam, cam_transform, vp)
        } else {
            return;
        };

        let shaders = world.get_resource::<ShaderStore>().unwrap();
        let meshes = world.get_resource::<MeshStore>().unwrap();
        let materials = world.get_resource::<MaterialStore>().unwrap();

        let lights: Vec<(Vec3, Vec3)> = world.query::<DirectionalLightComponent>()
            .map(|(_, l)| (l.direction, l.color))
            .collect();

        let ambient_light = world.get_resource::<AmbientLight>()
            .map(|a| a.color)
            .unwrap_or(Vec3::ZERO);

        for (entity, mesh_comp, mat_comp, transform) in world.query3::<MeshComponent, MaterialComponent, TransformComponent>() {
            if !world.has_component::<VisibleTag>(entity) { continue; }

            let Some(mat_data) = materials.get(mat_comp.material_id) else { continue };
            let Some(shader) = shaders.get(mat_data.shader_id) else { continue };
            let Some(gpu_mesh) = meshes.get(mesh_comp.mesh_id) else { continue };

            shader.bind(ctx);

            let model = transform.matrix();
            let prog = &shader.program;

            set_uniform_mat4(ctx, prog, "u_model", &model);
            set_uniform_mat4(ctx, prog, "u_view_proj", &view_proj);
            set_uniform_vec3(ctx, prog, "u_albedo", mat_data.albedo);
            set_uniform_vec3(ctx, prog, "u_ambient", ambient_light);
            for (i, (dir, color)) in lights.iter().enumerate() {
                set_uniform_vec3(ctx, prog, &format!("u_lights[{}].direction", i), *dir);
                set_uniform_vec3(ctx, prog, &format!("u_lights[{}].color", i), *color);
            }
            ctx.uniform1i(
                ctx.get_uniform_location(prog, "u_light_count").as_ref(),
                lights.len() as i32
            );

            ctx.bind_vertex_array(Some(&gpu_mesh.vao));
            ctx.draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                gpu_mesh.index_count,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                0
            );
            ctx.bind_vertex_array(None);
        }
    }
}

// PHYSICS SYSTEMS

const GRAVITY_ACCELERATION: f32 = 9.8;

pub struct GravitySystem {}

impl System for GravitySystem {
    fn run(&mut self, world: &mut World) {
        let dt = world.get_resource::<DeltaTime>().unwrap().0;

        let entities: Vec<Entity> = world.query::<RigidbodyComponent>()
            .map(|(e, _)| e)
            .collect();

        for entity in entities {
            let gravity_enabled = world.get_component::<RigidbodyComponent>(entity)
                .map(|rb| rb.gravity_enabled)
                .unwrap_or(false);

            if !gravity_enabled { continue; }

            // accumulate velocity from gravity
            if let Some(rb) = world.get_component_mut::<RigidbodyComponent>(entity) {
                rb.velocity += -Vec3::Y * GRAVITY_ACCELERATION * dt;
            }

            // integrate position from velocity
            let velocity = world.get_component::<RigidbodyComponent>(entity).unwrap().velocity;
            if let Some(t) = world.get_component_mut::<TransformComponent>(entity) {
                t.position += velocity * dt;
            }
        }
    }
}

fn sphere_aabb(sphere_center: Vec3, radius: f32, aabb_center: Vec3, half_extents: Vec3) -> bool {
    let closest = sphere_center.clamp(aabb_center - half_extents, aabb_center + half_extents);
    (sphere_center - closest).length_squared() <= radius * radius
}

fn intersects(a: &ColliderShapeComponent, a_pos: Vec3, b: &ColliderShapeComponent, b_pos: Vec3) -> bool {
    match (a, b) {
        (ColliderShapeComponent::Sphere { radius: ra }, ColliderShapeComponent::Sphere { radius: rb }) =>
            (a_pos - b_pos).length() < ra + rb,
        (ColliderShapeComponent::Aabb { half_extents: ha }, ColliderShapeComponent::Aabb { half_extents: hb }) =>
            (a_pos - b_pos).abs().cmple(*ha + *hb).all(),
        (ColliderShapeComponent::Sphere { radius }, ColliderShapeComponent::Aabb { half_extents }) =>
            sphere_aabb(a_pos, *radius, b_pos, *half_extents),
        (ColliderShapeComponent::Aabb { half_extents }, ColliderShapeComponent::Sphere { radius }) =>
            sphere_aabb(b_pos, *radius, a_pos, *half_extents),
    }
}

pub struct CollisionSystem {} impl System for CollisionSystem {
    fn run(&mut self, world: &mut World) {
        // 1. collect all colliders with positions
        let colliders: Vec<(Entity, Vec3, ColliderShapeComponent, bool)> =
            world.query2::<ColliderComponent, TransformComponent>()
                .map(|(e, c, t)| (e, t.position, c.shape.clone(), c.is_trigger))
                .collect();

        // 2. build octree
        let world_bounds = Aabb { center: Vec3::ZERO, half_extents: Vec3::splat(1000.0) };
        let mut octree = Octree::new(world_bounds, 8, 8);
        for (entity, pos, _, _) in &colliders {
            octree.insert(*entity, *pos);
        }

        // 3. broad + narrow phase
        let mut events = Vec::new();
        for (_i, (entity_a, pos_a, shape_a, trigger_a)) in colliders.iter().enumerate() {
            let query_bounds = shape_a.aabb(*pos_a);
            let mut candidates: Vec<Entity> = Vec::new();
            octree.root.query(&query_bounds, &mut candidates);

            for entity_b in candidates {
                // skip self and already-tested pairs
                if entity_b.id() <= entity_a.id() { continue; }

                if let Some((_, pos_b, shape_b, trigger_b)) =
                    colliders.iter().find(|(e, _, _, _)| *e == entity_b)
                {
                    if intersects(shape_a, *pos_a, shape_b, *pos_b) {
                        events.push(CollisionEvent {
                            entity_a: *entity_a,
                            entity_b,
                            is_trigger: *trigger_a || *trigger_b,
                        });
                    }
                }
            }
        }

        world.insert_resource(CollisionEvents { events });
    }
}

