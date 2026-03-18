use wasm_bindgen::JsCast;
use web_sys::{ HtmlCanvasElement, WebGl2RenderingContext };

pub struct Engine {
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,
}
impl Engine {
    pub fn new (canvas_id: &str) -> Engine {
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id(canvas_id).unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();

        context.fill_rect(10.0, 10.0, 50.0, 50.0);

        Engine { canvas, context }
    }
}