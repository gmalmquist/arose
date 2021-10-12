use chrono;
use js_sys::Math::sqrt;

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