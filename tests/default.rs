#![cfg(not(target_arch = "wasm32"))]

mod common;
use common::*;

#[test]
fn test_circomlib_testvector() {
    circomlib_testvector();
}
