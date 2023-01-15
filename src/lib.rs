
use js_sys::{Object, Reflect, Promise};
use wasm_bindgen::prelude::*;

pub fn build_response(status: u8, body: String) -> Object {
    let response_obj = Object::new();

    Reflect::set(&response_obj.as_ref(), &"statusCode".into(), &status.into()).unwrap();
    Reflect::set(&response_obj.as_ref(), &"body".into(), &body.into()).unwrap();

    return response_obj;
}

#[wasm_bindgen]
pub fn handler() -> Promise {
    let response = build_response(200, String::from("Rust lambda wasm response"));

    return Promise::resolve(response.as_ref());
}
