#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
pub extern "C" fn add_callback(f: fn(f64)) {}

enum State {
    Object(HashMap<&'static str, State>),
    String(&'static str),
    Number(f64),
    Bool(bool),
    Null,
}

#[wasm_bindgen]
pub struct Store {
    state: HashMap<&'static str, State>,
}

#[wasm_bindgen]
impl Store {
    pub fn new() -> Store {
        Store {
            state: HashMap::new(),
        }
    }
}
