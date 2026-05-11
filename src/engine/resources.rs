use std::collections::HashMap;
use glam::Vec3;
use ruwr_ecs::Entity;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};

const LAMBERTIAN_VERT: &str = include_str!("shaders/lambertian.vert");
const LAMBERTIAN_FRAG: &str = include_str!("shaders/lambertian.frag");

#[derive(Debug, Clone)]
pub struct DeltaTime (pub f32);

#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub color: Vec3,
}

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


// =================================================================== //
// =============================== IDs =============================== //
// =================================================================== //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);

// =================================================================== //
// =========================== MESH STORE ============================ //
// =================================================================== //
pub struct GpuMesh {
    pub vao: WebGlVertexArrayObject,
    pub vbo: WebGlBuffer,
    pub ebo: WebGlBuffer,
    pub index_count: i32,
}

pub struct MeshStore {
    meshes: HashMap<MeshId, GpuMesh>,
    sphere_cache: HashMap<(u32, u32), MeshId>,
    next_id: u32,
}

impl MeshStore {
    pub fn new () -> Self {
        Self { meshes: HashMap::new(), sphere_cache: HashMap::new(), next_id: 0 }
    }

    pub fn insert (&mut self, mesh: GpuMesh) -> MeshId {
        let id = MeshId(self.next_id);
        self.next_id += 1;
        self.meshes.insert(id, mesh);
        id
    }

    pub fn get (&self, id: MeshId) -> Option<&GpuMesh> {
        self.meshes.get(&id)
    }

    pub fn upload (&mut self, context: &WebGl2RenderingContext, vertices: &[f32], indices: &[u16]) -> MeshId {
        let vao = context.create_vertex_array().unwrap();
        context.bind_vertex_array(Some(&vao));

        let vbo = context.create_buffer().unwrap();
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
        unsafe {
            let view = js_sys::Float32Array::view(&vertices);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &view, 
                WebGl2RenderingContext::STATIC_DRAW
            );
        }

        let ebo = context.create_buffer().unwrap();
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        unsafe {
            let view = js_sys::Uint16Array::view(&indices);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &view,
                WebGl2RenderingContext::STATIC_DRAW
            );
        }

        let stride = (6 * std::mem::size_of::<f32>()) as i32;
        context.vertex_attrib_pointer_with_i32(
            0, 
            3, 
            WebGl2RenderingContext::FLOAT,
            false,
            stride,
            0
        );
        context.enable_vertex_attrib_array(0);
        context.vertex_attrib_pointer_with_i32(
            1,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            stride,
            12
        );
        context.enable_vertex_attrib_array(1);

        context.bind_vertex_array(None);

        self.insert(GpuMesh { vao, vbo, ebo, index_count: indices.len() as i32 })
    }

    pub fn load_sphere (&mut self, context: &WebGl2RenderingContext, stacks: u32, slices: u32) -> MeshId {
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for i in 0..=stacks {
            let phi = std::f32::consts::PI * i as f32 / stacks as f32;
            for j in 0..=slices {
                let theta = 2.0 * std::f32::consts::PI * j as f32 / slices as f32;
                let x = phi.sin() * theta.cos();
                let y = phi.cos();
                let z = phi.sin() * theta.sin();
                vertices.extend_from_slice(&[x, y, z, x, y, z]);
            }
        }

        for i in 0..=stacks {
            for j in 0..=slices {
                let a = (i* (slices + 1) + j) as u16;
                let b = a + slices as u16 + 1;
                indices.extend_from_slice(&[a, b, a + 1, b, b + 1, a + 1]);
            }
        }

        self.upload(context, &vertices, &indices)
    }

    pub fn load_line (
        &mut self,
        context: &WebGl2RenderingContext,
        start: Vec3,
        end: Vec3,
        width: f32,
    ) -> MeshId {
        let dir = (end - start).truncate().normalize();
        let perp = Vec3::new(-dir.y, dir.x, 0.0) * (width / 2.0);
        let normal = Vec3::Z;

        let v = [
            start + perp,
            start - perp,
            end   + perp,
            end   - perp,
        ];

        let mut vertices: Vec<f32> = Vec::new();
        for pos in &v {
            vertices.extend_from_slice(&[pos.x, pos.y, pos.z]);
            vertices.extend_from_slice(&[normal.x, normal.y, normal.z]);
        }

        let indices: Vec<u16> = vec![0, 1, 2, 1, 3, 2];

        self.upload(context, &vertices, &indices)
    }

    pub fn load_polyline (
        &mut self,
        context: &WebGl2RenderingContext,
        points: &[Vec3],
        width: f32
    ) -> MeshId {
        assert!(points.len() >= 2);
        let n = points.len();
        let normal = Vec3::Z;

        let perps: Vec<Vec3> = (0..n).map(|i| {
            let dir = if i == 0 {
                (points[1] - points[0]).truncate().normalize()
            } else if i == n - 1 {
                (points[n-1] - points[n-2]).truncate().normalize()
            } else {
                let d0 = (points[i] - points[i-1]).truncate().normalize();
                let d1 = (points[i+1] - points[i]).truncate().normalize();
                (d0 + d1).normalize()
            };
            Vec3::new(-dir.y, dir.x, 0.0) * (width / 2.0)
        }).collect();

        let mut vertices: Vec<f32> = Vec::new();
        for (i, &p) in points.iter().enumerate() {
            let top = p + perps[i];
            let bot = p - perps[i];
            vertices.extend_from_slice(&[top.x, top.y, top.z, normal.x, normal.y, normal.z]);
            vertices.extend_from_slice(&[bot.x, bot.y, bot.z, normal.x, normal.y, normal.z]);
        }

        let mut indices: Vec<u16> = Vec::new();
        for i in 0..n-1 {
            let b = (i * 2) as u16;
            indices.extend_from_slice(&[b, b+1, b+2, b+1, b+3, b+2]);
        }

        self.upload(context, &vertices, &indices)
    }

    pub fn load_bezier (
        &mut self,
        context: &WebGl2RenderingContext,
        p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3,
        width: f32,
        segments: u32,
    ) -> MeshId {
        let points: Vec<Vec3> = (0..=segments).map(|i| {
            let t = i as f32 / segments as f32;
            let mt = 1.0 - t;
            p0 * (mt*mt*mt) + p1 * (3.0*mt*mt*t) + p2 * (3.0*mt*t*t) + p3 * (t*t*t)
        }).collect();

        self.load_polyline(context, &points, width)
    }

    pub fn get_or_create_sphere(&mut self, context: &WebGl2RenderingContext, stacks: u32, slices: u32) -> MeshId {
        if let Some(&id) = self.sphere_cache.get(&(stacks, slices)) {
            return id;
        }
        let id = self.load_sphere(context, stacks, slices);
        self.sphere_cache.insert((stacks, slices), id);
        id
    }
}

// =================================================================== //
// ========================= MATERIAL STORE ========================== //
// =================================================================== //

pub struct MaterialData {
    pub shader_id: ShaderId,
    pub albedo: Vec3,
}

pub struct MaterialStore {
    materials: HashMap<MaterialId, MaterialData>,
    next_id: u32,
}

impl MaterialStore {
    pub fn new() -> Self {
        Self { materials: HashMap::new(), next_id: 0 }
    }

    pub fn insert(&mut self, data: MaterialData) -> MaterialId {
        let id = MaterialId(self.next_id);
        self.next_id += 1;
        self.materials.insert(id, data);
        id
    }

    pub fn get(&self, id: MaterialId) -> Option<&MaterialData> {
        self.materials.get(&id)
    }

    pub fn get_mut(&mut self, id: MaterialId) -> Option<&mut MaterialData> {
        self.materials.get_mut(&id)
    }
}

// =================================================================== //
// =========================== SHADER STORE ========================== //
// =================================================================== //
pub struct GpuShader {
    pub program: WebGlProgram,
}

impl GpuShader {
    pub fn bind (&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
    }
}

pub struct ShaderStore {
    lambertian_id: Option<ShaderId>,
    shaders: HashMap<ShaderId, GpuShader>,
    next_id: u32,
}

impl ShaderStore {
    pub fn new () -> Self {
        Self {
            lambertian_id: None,
            shaders: HashMap::new(),
            next_id: 0
        }
    }

    pub fn load_defaults (&mut self, context: &WebGl2RenderingContext) {
        self.lambertian_id = Some(self.compile(context, LAMBERTIAN_VERT, LAMBERTIAN_FRAG).expect("Error compiling lambertian shader"))
    }

    pub fn compile (
        &mut self,
        context: &WebGl2RenderingContext,
        vert_src: &str,
        frag_src: &str,
    ) -> Result<ShaderId, String> {
        let program = link_program(context, vert_src, frag_src)?;
        let id = ShaderId(self.next_id);
        self.next_id += 1;
        self.shaders.insert(id, GpuShader { program });
        Ok(id)
    }

    pub fn get (&self, id: ShaderId) -> Option<&GpuShader> {
        self.shaders.get(&id)
    }

    pub fn lambertian_id (&self) -> Option<ShaderId> {
        self.lambertian_id
    }

    pub fn lambertian (&self) -> Option<&GpuShader> {
        match self.lambertian_id {
            None => None,
            Some(id) => self.shaders.get(&id)
        }
    }
}

// Shader compilation

fn compile_shader (context: &WebGl2RenderingContext, shader_type: u32, src: &str) -> Result<web_sys::WebGlShader, String> {
    let shader = context.create_shader(shader_type).ok_or("failed to create shader")?;
    context.shader_source(&shader, src);
    context.compile_shader(&shader);

    if context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap_or_default())
    }
}

fn link_program (context: &WebGl2RenderingContext, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, String> {
    let vert = compile_shader(context, WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;
    let frag = compile_shader(context, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;

    let program = context.create_program().ok_or("failed to create program")?;
    context.attach_shader(&program, &vert);
    context.attach_shader(&program, &frag);
    context.link_program(&program);

    if context.get_program_parameter (&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).unwrap_or_default())
    }
}

// COLLISION RESOURCES

pub struct Aabb {
    pub center: Vec3,
    pub half_extents: Vec3
} impl Aabb {
    pub fn intersects (&self, other: &Aabb) -> bool {
        (self.center - other.center).abs()
            .cmple(self.half_extents + other.half_extents)
            .all()
    }

    fn octant (&self, p: Vec3) -> usize {
        let d = p - self.center;
        (d.x >= 0.0) as usize
            | (((d.y >= 0.0) as usize) << 1)
            | (((d.z >= 0.0) as usize) << 2)
    }

    fn child_bounds (&self, octant: usize) -> Aabb {
        let qe = self.half_extents * 0.5;
        let offset = Vec3::new(
            if octant & 1 != 0 { qe.x } else { -qe.x },
            if octant & 2 != 0 { qe.y } else { -qe.y },
            if octant & 4 != 0 { qe.z } else { -qe.z }
        );

        Aabb { center: self.center + offset, half_extents: qe }
    }
}

pub struct OctreeNode {
    bounds: Aabb,
    children: Option<Box<[OctreeNode; 8]>>,
    entities: Vec<(Entity, Vec3)>
} impl OctreeNode {
    fn new (bounds: Aabb) -> Self {
        Self { bounds, children: None, entities: Vec::new() }
    }

    fn insert (&mut self, entity: Entity, pos: Vec3, depth: u32, max_depth: u32, max_per_node: usize) {
        if depth == max_depth || (self.children.is_none() && self.entities.len() < max_per_node) {
            self.entities.push((entity, pos));
            return;
        }

        if self.children.is_none() {
            self.children = Some(Box::new(std::array::from_fn(|i| {
                OctreeNode::new(self.bounds.child_bounds(i))
            })));
            let existing = std::mem::take(&mut self.entities);
            for (e, p) in existing {
                let octant = self.bounds.octant(p);
                self.children.as_mut().unwrap()[octant]
                    .insert(e, p, depth + 1, max_depth, max_per_node);
            }
        }

        let octant = self.bounds.octant(pos);
        self.children.as_mut().unwrap()[octant]
            .insert(entity, pos, depth + 1, max_depth, max_per_node);
    }

    pub fn query (&self, bounds: &Aabb, out: &mut Vec<Entity>) {
        if !self.bounds.intersects(bounds) { return; }
        out.extend(self.entities.iter().map(|(e, _)| *e));
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(bounds, out);
            }
        }
    }
}

pub struct Octree {
    pub root: OctreeNode,
    max_dept: u32,
    max_per_node: usize
} impl Octree {
    pub fn new (bounds: Aabb, max_depth: u32, max_per_node: usize) -> Self {
        Self { root: OctreeNode::new(bounds), max_dept: max_depth, max_per_node }
    }

    pub fn insert (&mut self, entity: Entity, pos: Vec3) {
        self.root.insert(entity, pos, 0, self.max_dept, self.max_per_node);
    }
}

pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub is_trigger: bool,
}

pub struct CollisionEvents {
    pub events: Vec<CollisionEvent>,
}