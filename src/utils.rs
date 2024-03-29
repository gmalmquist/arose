use chrono;
use js_sys::Math::{exp, sqrt, pow, log2};
use wasm_bindgen::__rt::core::f64::consts::PI;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    //#[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn current_time_millis() -> f64 {
    chrono::Utc::now().timestamp_millis() as f64
}

pub fn lerpf(a: f64, b: f64, s: f64) -> f64 {
    (1. - s) * a + s * b
}

pub fn bezierf2(a: f64, b: f64, c: f64, s: f64) -> f64 {
    lerpf(
        lerpf(a, b, s),
        lerpf(b, c, s),
        s,
    )
}

pub fn gaussian_blur(sigma: f64, x: f64, y: f64) -> f64 {
    // https://en.wikipedia.org/wiki/Gaussian_blur
    1. / (2. * PI * sigma * sigma) * exp(-(x * x + y * y) / (2. * sigma * sigma))
}

pub fn exp_smin(a: f64, b: f64, k: f64) -> f64 {
    let res = pow(2., -k * a) + pow(2., -k * b);
    return -log2(res) / k;
}

//
// #[cfg(test)]
// mod tests {
//     use crate::utils::longest_common_prefix;
//
//     #[test]
//     fn common_prefix() {
//         assert_eq!("", longest_common_prefix(&vec![]));
//         assert_eq!("apple", longest_common_prefix(&vec!["apple"]));
//         assert_eq!("apple", longest_common_prefix(&vec!["apple", "apple pie"]));
//         assert_eq!("apple", longest_common_prefix(&vec!["apple"]));
//         assert_eq!("ap", longest_common_prefix(&vec!["apple", "apple pie", "apricot"]));
//     }
// }