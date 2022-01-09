extern crate wasm_bindgen;

use core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn points(score: &[i32], progress: f64) -> String {
    let point_list: Vec<core::Point> = core::calc(score, progress);
    let bar: Vec<String> = point_list
        .iter()
        .map(|point| serde_json::to_string(&point).unwrap())
        .collect();
    bar.join(",")
}

#[wasm_bindgen]
pub fn svg(score: &[i32], progress: f64) -> String {
    core::radar(score, progress)
}
