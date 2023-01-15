
use js_sys::{Object, Reflect};

pub fn build_response(status: u8, body: String) -> Object {
    let response_obj = Object::new();

    Reflect::set(&response_obj.as_ref(), &"statusCode".into(), &status.into()).unwrap();
    Reflect::set(&response_obj.as_ref(), &"body".into(), &body.into()).unwrap();

    return response_obj;
}
