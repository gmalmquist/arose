use wasm_bindgen::prelude::*;

use crate::threed::Vec3;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn find_closest_point<F: Fn(f64) -> Vec3>(point: &Vec3, curve: F, debug: bool) -> f64 {
    let samples = 100;

    // (s, distance^2)
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

    let left = (closest_sample.0 - 1.0 / (samples as f64)).max(0.);
    let right = (closest_sample.0 + 1.0 / (samples as f64)).min((1.));
    let mut left = (left, curve(left).dist2(point));
    let mut right = (right, curve(right).dist2(point));

    // binary search
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

        if debug {
            log(&format!("L: {:?}, R: {:?}, M: {:?}", left, right, (ms, md)));
        }

        if ld < rd {
            right = (ms, md);
        } else {
            left = (ms, md);
        }
    }

    if debug {
        log(&format!("F: L: {:?} R: {:?}", left, right));
    }

    left.0 / 2. + right.0 / 2.
}
