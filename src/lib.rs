mod utils;

use wasm_bindgen::prelude::*;
use std::string;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn getInfo() -> js_sys::Promise;
}

#[wasm_bindgen]
pub fn test_return() -> String {
    return String::from("rv");
}

#[wasm_bindgen]
pub async fn go_action() -> String {
    let promise = getInfo();
    let lightdinfo = wasm_bindgen_futures::JsFuture::from(promise).await;

    match lightdinfo {
        Ok(js) => {
            return js.as_string().unwrap();
        },
        Err(_e) => {
            return String::from("error");
        }
    }
}
