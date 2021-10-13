use crate::utils::{current_time_millis, lerpf};
use crate::threed::Vec3;
use crate::color::Color;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::__rt::core::f64::consts::PI;
use js_sys::Math;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const MOUSE_RADIUS: f64 = 10.;

#[wasm_bindgen]
struct Handle {
    pos: Vec3,
    hovered: bool,
}

impl Handle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec3::new(x, y, 0.),
            hovered: false,
        }
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    pub fn contains_mouse(&self, mouse: &Vec3) -> bool {
        self.pos.dist2(mouse) < MOUSE_RADIUS * MOUSE_RADIUS
    }
}

#[wasm_bindgen]
pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
    g: web_sys::CanvasRenderingContext2d,
    width: f64,
    height: f64,
    mouse: Vec3,
    handles: Vec<Handle>,
    is_setup: bool,
    dragging_handle: Option<usize>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        let g = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self {
            canvas,
            g,
            width: 0.,
            height: 0.,
            mouse: Vec3::zero(),
            handles: vec![],
            is_setup: false,
            dragging_handle: None,
        }
    }

    pub fn setup(&mut self) {
        self.handles.push(Handle::new(354., 591.));
        self.handles.push(Handle::new(395., 410.));
        self.handles.push(Handle::new(259., 399.));
        self.handles.push(Handle::new(310., 211.));

        self.is_setup = true;
    }

    pub fn update(&mut self) {
        self.width = self.canvas.width() as f64;
        self.height = self.canvas.height() as f64;

        if !self.is_setup {
            self.setup();
        }

        self.g.clear_rect(0., 0., self.width, self.height);
        self.set_fill_color(&Color::white());
        self.g.fill_rect(0., 0., self.width, self.height);

        self.render_handle_bezier();
        self.render_control_lines();

        for i in 0..self.handles.len() {
            self.render_handle(&self.handles[i]);
        }

        self.render_rose();
    }

    fn render_rose(&self) {
        self.set_stroke_color(&Color::black());

        // stem
        let N = 1000;
        for i in 0..N {
            let s0 = (i as f64) / (N as f64);
            let s1 = ((i + 1) as f64) / (N as f64);
            let pt = self.stem_bezier(s0);
            let pt2 = self.stem_bezier(s1);
            let tangent = (&pt2 - &pt).unit();
            let normal = &tangent.rz90();

            let width =
                lerpf(
                    lerpf(0., 5., (s0 * 25.).min(1.)),
                    4.,
                    s0,
                );

            let left = pt.clone().sadd_vec_mut(-width, &normal);
            let right = pt.clone().sadd_vec_mut(width, &normal);

            let K = 10;
            for j in 0..K {
                let f = (j as f64) / (K as f64);
                let pt = Vec3::lerp(&left, &right, f);
                self.set_fill_color(&Color::white()
                    .scale(lerpf(0., 0.8, Math::sin(f * PI / 2.))));
                self.g.fill_rect(pt.x, pt.y, 1., 1.);
            }

            self.g.set_line_width(2.);
            self.g.begin_path();
            self.g.move_to(left.x, left.y);
            self.g.line_to(pt2.x - width * normal.x, pt2.y - width * normal.y);
            self.g.stroke();
            self.g.close_path();

            self.g.begin_path();
            self.g.move_to(right.x, right.y);
            self.g.line_to(pt2.x + width * normal.x, pt2.y + width * normal.y);
            self.g.stroke();
            self.g.close_path();
        }
    }

    fn stem_bezier(&self, s: f64) -> Vec3 {
        Vec3::bezier3(
            &self.handles[0].pos,
            &self.handles[1].pos,
            &self.handles[2].pos,
            &self.handles[3].pos,
            s,
        )
    }

    fn render_control_lines(&self) {
        self.g.begin_path();
        self.g.set_line_width(2.);
        for i in 0..self.handles.len() {
            let pos = &self.handles[i].pos;
            if i == 0 {
                self.g.move_to(pos.x, pos.y);
            } else {
                self.g.line_to(pos.x, pos.y);
            }
        }
        self.g.stroke();
        self.g.close_path();
    }

    fn render_handle_bezier(&self) {
        self.g.set_line_width(1.);
        self.set_stroke_color(&Color::black());

        self.g.begin_path();
        self.render_curve(|s| self.stem_bezier(s));
        self.g.stroke();
        self.g.close_path();

        self.g.begin_path();
        self.render_curve(|s| Vec3::bezier2(
            &self.handles[0].pos,
            &self.handles[1].pos,
            &self.handles[2].pos,
            s,
        ));
        self.g.stroke();
        self.g.close_path();

        self.g.begin_path();
        self.render_curve(|s| Vec3::bezier2(
            &self.handles[1].pos,
            &self.handles[2].pos,
            &self.handles[3].pos,
            s,
        ));
        self.g.stroke();
        self.g.close_path();
    }

    fn render_curve<C: Fn(f64) -> Vec3>(&self, curve: C) {
        let N = 1000;
        for i in 0..N {
            let s = (i as f64) / (N as f64);
            let pt = curve(s);
            if i == 0 {
                self.g.move_to(pt.x, pt.y);
            } else {
                self.g.line_to(pt.x, pt.y);
            }
        }
    }

    fn render_handle(&self, handle: &Handle) {
        let rh = |rad: f64| {
            self.g.begin_path();
            self.set_fill_color(&Color::white());
            self.circle(&handle.pos, rad);
            self.g.fill();
            self.g.close_path();

            self.g.begin_path();
            self.set_stroke_color(&Color::black());
            self.g.set_line_width(2.);
            self.circle(&handle.pos, rad);
            self.g.stroke();
            self.g.close_path();
        };

        if handle.hovered {
            rh(MOUSE_RADIUS);
        } else {
            rh(MOUSE_RADIUS / 2.);
        }
    }

    pub fn handle_key_down(&mut self, chr: &str) {
        //log(&format!("keydown: '{}'", chr));
    }

    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.update_mouse(x, y);
        if let Some(i) = self.dragging_handle {
            self.handles[i].pos = self.mouse.clone();
        }
        for i in 0..self.handles.len() {
            let hovering = self.handles[i].contains_mouse(&self.mouse);
            self.handles[i].set_hovered(hovering);
        }
    }

    pub fn handle_mouse_down(&mut self, x: f64, y: f64) {
        self.update_mouse(x, y);

        for i in 0..self.handles.len() {
            if self.handles[i].contains_mouse(&self.mouse) {
                self.dragging_handle = Some(i);
                break;
            }
        }

        log(&format!("handles: {}", self.handles.iter()
            .map(|h| h.pos.to_string())
            .collect::<Vec<_>>()
            .join(", ")));
    }

    pub fn handle_mouse_up(&mut self, x: f64, y: f64) {
        self.update_mouse(x, y);

        if let Some(i) = self.dragging_handle.clone() {
            self.handles[i].pos = self.mouse.clone();
            self.dragging_handle = None;
        }
    }

    fn circle(&self, pos: &Vec3, radius: f64) {
        self.g.ellipse(pos.x, pos.y, radius, radius, 0., 0., PI * 2.);
    }

    fn set_fill_color(&self, color: &Color) {
        self.g.set_fill_style(&color.as_hexstring().into())
    }

    fn set_stroke_color(&self, color: &Color) {
        self.g.set_stroke_style(&color.as_hexstring().into())
    }

    fn update_mouse(&mut self, x: f64, y: f64) {
        let (x, y) = self.window_to_canvas((x, y));
        self.mouse = Vec3::new(x, y, 0.);
    }

    fn window_to_canvas(&self, w: (f64, f64)) -> (f64, f64) {
        (w.0 - self.canvas.offset_left() as f64, w.1 - self.canvas.offset_top() as f64)
    }
}
