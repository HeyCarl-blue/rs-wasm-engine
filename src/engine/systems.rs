use glam::{Mat4, Quat, Vec3};
use ruwr_ecs::System;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::engine::components::{ActiveCamera, Camera2D, Camera3D, DirectionalLight, Material, Mesh, Transform, Viewport, Visible};
use crate::engine::resources::{AmbientLight, MaterialStore, MeshStore, ShaderStore};

use ruwr_ecs::World;

fn get_view_proj (camera: &Camera3D, transform: &Transform, aspect_ratio: f32) -> Mat4 {
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

fn get_ortho_view_proj (camera: &Camera2D, transform: &Transform, viewport: &Viewport) -> Mat4 {
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

        let Some(cam_entity) = world.query::<ActiveCamera>().next().map(|(e, _)| e) else { return };
        let Some(cam_transform) = world.get_component::<Transform>(cam_entity) else { return };
        let view_proj = if let Some(cam) = world.get_component::<Camera3D>(cam_entity) {
            get_view_proj(cam, cam_transform, aspect_ratio)
        } else if let Some(cam) = world.get_component::<Camera2D>(cam_entity) {
            get_ortho_view_proj(cam, cam_transform, vp)
        } else {
            return; // active camera has no camera component
        };

        let shaders = world.get_resource::<ShaderStore>().unwrap();
        let meshes = world.get_resource::<MeshStore>().unwrap();
        let materials = world.get_resource::<MaterialStore>().unwrap();

        let lights: Vec<(Vec3, Vec3)> = world.query::<DirectionalLight>()
            .map(|(_, l)| (l.direction, l.color))
            .collect();

        let ambient_light = world.get_resource::<AmbientLight>()
            .map(|a| a.color)
            .unwrap_or(Vec3::ZERO);

        for (entity, mesh_comp, mat_comp, transform) in world.query3::<Mesh, Material, Transform>() {
            if !world.has_component::<Visible>(entity) { continue; }

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
