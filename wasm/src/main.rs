use wasm_bindgen::prelude::*;
use web_sys::console;

mod key_generator;
mod ui;

fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"Starting application...".into());
    
    ui::KeyGeneratorUI::new().expect("Failed to initialize UI");
    console::log_1(&"UI initialized successfully".into());
} 
