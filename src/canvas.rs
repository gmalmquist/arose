use crate::utils::current_time_millis;
use crate::threed::Vec3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        Self {
            canvas
        }
    }

    pub fn update(&mut self) {
        log("hi?");
    }

    pub fn handle_key_down(&mut self, chr: &str) {
        log(&format!("keydown: '{}'", chr));
    }

    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        log(&format!("move: {:?}", self.window_to_canvas((x, y))));
    }

    pub fn handle_mouse_down(&mut self, x: f64, y: f64) {
        log(&format!("down: '{} {}'", x, y));
    }

    pub fn handle_mouse_up(&mut self, x: f64, y: f64) {
        log(&format!("up: '{} {}'", x, y));
    }

    fn window_to_canvas(&self, w: (f64, f64)) -> (f64, f64) {
        (w.0 - self.canvas.offset_left() as f64, w.1 - self.canvas.offset_top() as f64)
    }
}
