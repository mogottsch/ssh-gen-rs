// library file to access code in benchmark
pub mod core;
pub mod worker;
pub mod wasm;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init() {
    // Initialize panic hook for better error messages in WASM
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
