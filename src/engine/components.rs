use engine_wasm_rs_macros::component;

#[derive(Debug, Clone, PartialEq)]
#[component]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // quaternion (x, y, z, w)
    pub scale: [f32; 3],
}