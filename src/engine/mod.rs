pub mod components;

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use engine_wasm_rs_ecs::{Scheduler, World};

pub struct Engine {
    pub canvas: HtmlCanvasElement,
    pub context: WebGl2RenderingContext,
    pub world: World,
    pub scheduler: Scheduler,
}

impl Engine {
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

        Engine {
            canvas,
            context,
            world: World::new(),
            scheduler: Scheduler::new(),
        }
    }

    /// Advance one simulation tick: run all registered systems.
    pub fn tick(&mut self) {
        self.scheduler.run(&mut self.world);
    }

    /// Clear the colour buffer with a solid colour.
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
        self.context
            .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}