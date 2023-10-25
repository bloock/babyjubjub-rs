#![cfg(target_arch = "wasm32")]

mod common;
use common::*;
use wasm_bindgen_test::wasm_bindgen_test;
// wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_circomlib_testvector() {
    circomlib_testvector();
}
