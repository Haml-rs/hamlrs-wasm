#![feature(use_extern_macros)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate haml;

#[wasm_bindgen]
pub fn to_html(haml: &str) -> String {
    haml::to_html(haml)
}
