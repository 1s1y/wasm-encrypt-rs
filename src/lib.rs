extern crate core;

use wasm_bindgen::prelude::*;
use base64;

mod crypto;

const ADD_CONSTANT: i32 = 24;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut s = String::from("Hello, ");
    s.push_str(name);
    log(&s);
    alert(&s);
}

// exported functions
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b + ADD_CONSTANT;
}

// exported functions
#[wasm_bindgen]
pub fn aaa(content: &str) {
    let bytes = base64::decode(content).unwrap();
    let decoded = String::from_utf8(bytes).unwrap();
    log(&decoded);
}
