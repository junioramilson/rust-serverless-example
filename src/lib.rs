
use js_sys::{Promise};
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn handler() -> Promise {
    let response = utils::build_response(200, String::from("Rust lambda wasm response"));

    return Promise::resolve(response.as_ref());
}
