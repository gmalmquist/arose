use js_sys::Math;
use wasm_bindgen::__rt::core::f64::consts::PI;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::color::Color;
use crate::flower::Flower;
use crate::sdf::{find_closest_point, raycast, sdf_sphere};
use crate::threed::{Ray, Vec3};
use crate::utils::{current_time_millis, lerpf, gaussian_blur};

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
    is_click_frame: bool,
    user_event: bool,
    render_pixel: usize,
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
            is_click_frame: false,
            user_event: false,
            render_pixel: 0,
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
            self.user_event = true;
        }

        if self.user_event {
            self.g.clear_rect(0., 0., self.width, self.height);
            self.set_fill_color(&Color::white());
            self.g.fill_rect(0., 0., self.width, self.height);

            //self.render_handle_bezier();

            self.render_control_lines();

            for i in 0..self.handles.len() {
                self.render_handle(&self.handles[i]);
            }

            self.render_pixel = 0;
        }

        if !self.user_event && self.render_pixel == 0 {
            return;
        }
        self.user_event = false;

        let start_time_millis = current_time_millis() as u64;
        let deadline = start_time_millis + 10u64; // 10ms in the future

        let flower = {
            let mut flower = Flower::new();
            flower.update_controls(&self.handles.iter()
                .map({ |h| h.pos.clone() })
                .collect());
            flower
        };

        let length = self.width as usize * self.height as usize;
        while (current_time_millis() as u64) < deadline {
            let y = self.height - 1. - (self.render_pixel / (self.width as usize)) as f64;
            let x = (self.render_pixel % (self.width as usize)) as f64;
            if x > self.width * 0.99 {
                self.set_fill_color(&Color::black());
                self.g.fill_rect(x, y, 1., 1.);
            } else {
                // render outline
                // TODO anti-alias.
                if let Some(hit) = raycast(
                    &Ray::new(Vec3::new(x, y, -10.), Vec3::forward()),
                    1000.,
                    &|s| flower.distance(s) - 2.,
                ) {
                    self.set_fill_color(&Color::black());
                    self.g.fill_rect(x, y, 1., 1.);
                }

                // render shaded rose
                self.render_rose(&flower, x, y);
            }
            self.render_pixel = {
                let next = self.render_pixel + 1;
                if next >= length {
                    next - length
                } else {
                    next
                }
            };
        }

        self.is_click_frame = false;
    }

    fn render_rose(&self, flower: &Flower, x: f64, y: f64) {
        let light_pos = Vec3::new(
            self.width * 0.75,
            self.height / 2.,
            -self.width * 0.25,
        );

        let mut result_color = Color::black();
        let mut hits = 0;

        // blur sample step
        let eps = 0.5;

        // blur controls
        let sigma = 1.;

        // NB: set to 3 for nice blurring, 1 for speed
        let window_size = 1;

        let mut deltas = vec![];

        for ix in 0..window_size {
            let dx = (ix - window_size / 2) as f64;
            for iy in 0..window_size {
                let dy = (iy - window_size / 2) as f64;
                deltas.push((dx, dy));
            }
        }

        let total_alpha = deltas.iter()
            .map({ |(dx, dy)| gaussian_blur(sigma, *dx, *dy) })
            .fold(0., |a, b| { a + b });

        for (dx, dy) in deltas {
            let g = gaussian_blur(sigma, dx, dy) / total_alpha;

            let pt = Vec3::new(
                x + dx as f64 * eps,
                y + dy as f64 * eps,
                0.,
            );

            if let Some(hit) = raycast(
                &Ray::new(Vec3::new(pt.x, pt.y, -10.), Vec3::forward()),
                1000.,
                &|s| flower.distance(s),
            ) {
                let light_dir = (&light_pos - &hit.point).unit();
                let diffuse = hit.normal.dot(&light_dir).max(0.);
                let ambient = 0.2;
                let albedo = ambient + diffuse;
                let c = Color::white().scale(albedo);

                result_color = &result_color + &(&c * g);
                hits += 1;
            } else {
                result_color = &result_color + &Color::white().scale(g);
            }
        }

        if hits == 0 {
            return;
        }

        //result_color = &result_color + &Color::white().scale(1.0 - total_alpha);

        self.set_fill_color(&result_color);
        self.g.fill_rect(x, y, 1., 1.);
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
        self.user_event = true;
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

        self.user_event = true;
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

        self.user_event = true;
    }

    pub fn handle_mouse_up(&mut self, x: f64, y: f64) {
        self.update_mouse(x, y);

        if let Some(i) = self.dragging_handle.clone() {
            self.handles[i].pos = self.mouse.clone();
            self.dragging_handle = None;
        }
        self.is_click_frame = true;
        self.user_event = true;
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
