use wasm_bindgen::{ prelude::*, JsCast };
use web_sys::{ HtmlCanvasElement, WebGl2RenderingContext };

#[wasm_bindgen]
pub struct Engine {
    context: WebGl2RenderingContext,
}
#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new (canvas_id: &str) -> Engine {
        let canvas_option = web_sys::window().unwrap().document().unwrap().get_element_by_id(canvas_id);
        let canvas = match canvas_option {
            None => panic!("no canvas of id: {canvas_id} found"),
            Some(c) => c.dyn_into::<HtmlCanvasElement>().unwrap()
        };

        let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();


        Engine { context }
    }

    pub fn fill (&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}