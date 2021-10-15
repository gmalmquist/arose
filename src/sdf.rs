use wasm_bindgen::prelude::*;

use crate::threed::{Ray, Vec3};
use js_sys::Math::max;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone, Debug)]
pub struct RayHit {
    pub point: Vec3,
    pub normal: Vec3,
}

fn raymarch<S: Fn(&Vec3) -> f64>(ray: &Ray, maxdist: f64, scene: &S) -> Option<f64> {
    let eps = 0.001;

    let mut traveled = 0.;
    let mut sd = scene(&ray.origin);
    while sd > eps && traveled < maxdist {
        traveled += sd;
        sd = scene(&ray.sample(traveled));
    }

    if sd <= eps {
        return Some(traveled)
    }

    None
}

pub fn raycast<S: Fn(&Vec3) -> f64>(ray: &Ray, maxdist: f64, scene: &S) -> Option<RayHit> {
    let distance = raymarch(ray, maxdist, scene);
    if distance.is_none() {
        return None;
    }
    let distance = distance.unwrap();

    let eps = 0.001;
    let x = Vec3::right().scale_uniform_mut(eps);
    let y = Vec3::up().scale_uniform_mut(eps);
    let z = Vec3::forward().scale_uniform_mut(eps);

    let point = ray.sample(distance);
    let normal = Vec3::new(
        scene(&(&point + &x)) - scene(&(&point - &x)),
        scene(&(&point + &y)) - scene(&(&point - &y)),
        scene(&(&point + &z)) - scene(&(&point - &z))
    ).unit();

    Some(RayHit {
        point,
        normal
    })
}

pub fn find_closest_point<F: Fn(f64) -> Vec3>(point: &Vec3, curve: F) -> f64 {
    // approximate the closest point by sampling uniformly along the curve
    // this is more than enough samples to accurately approximate the closest point for a cubic
    // (when coupled with the binary search after), but may not be enough for other curves (e.g.
    // long splines).
    let samples = 20;

    // (interpolation parameter, distance^2)
    let mut closest_sample: Option<(f64, f64)> = None;

    for i in 0..samples {
        let s = (i as f64) / ((samples - 1) as f64);
        let pt = curve(s);
        let d = pt.dist2(point);

        if closest_sample.is_none() || closest_sample.clone().unwrap().1 > d {
            closest_sample = Some((s, d));
        }
    }

    let closest_sample = closest_sample.unwrap();

    // binary search in the neighborhood of the closest sampled point
    let left = (closest_sample.0 - 1.0 / (samples as f64)).max(0.);
    let right = (closest_sample.0 + 1.0 / (samples as f64)).min((1.));
    let mut left = (left, curve(left).dist2(point));
    let mut right = (right, curve(right).dist2(point));

    let eps = 0.001;
    while right.0 - left.0 > eps {
        // midpoint interpolation parameter
        let ms = left.0 / 2. + right.0 / 2.;

        // midpoint point
        let pt = curve(ms);

        // midpoint distance to argument point
        let md = pt.dist2(point);

        // left point distance to argument point
        let ld = left.1;

        // right point distance to argument point
        let rd = right.1;

        if ld < rd {
            right = (ms, md);
        } else {
            left = (ms, md);
        }
    }

    left.0 / 2. + right.0 / 2.
}

pub fn sdf_curve<C: Fn(f64) -> Vec3>(curve: &C, thickness: f64, pt: &Vec3) -> f64 {
    let s = find_closest_point(pt, curve);
    curve(s).dist(pt) - thickness
}

pub fn sdf_sphere(origin: Vec3, radius: f64) -> Box<dyn Fn(&Vec3) -> f64> {
    Box::new(move |pt: &Vec3| pt.dist(&origin) - radius)
}
