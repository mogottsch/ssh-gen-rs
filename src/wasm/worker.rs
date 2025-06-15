use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[wasm_bindgen]
pub struct KeyGeneratorWorker {
    worker: Worker,
    is_initialized: bool,
    stop_flag: Arc<AtomicBool>,
}

#[wasm_bindgen]
impl KeyGeneratorWorker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<KeyGeneratorWorker, JsValue> {
        // Check if Web Workers are supported
        if !Worker::is_supported() {
            return Err(JsValue::from_str("Web Workers not supported"));
        }

        let worker = Worker::new_with_str("worker.js")
            .map_err(|_| JsValue::from_str("Failed to create worker"))?;

        Ok(KeyGeneratorWorker {
            worker,
            is_initialized: false,
            stop_flag: Arc::new(AtomicBool::new(false)),
        })
    }

    #[wasm_bindgen]
    pub fn start_generation(&mut self, patterns: Vec<String>) -> Result<(), JsValue> {
        if self.is_initialized {
            return Err(JsValue::from_str("Worker already initialized"));
        }

        // Validate patterns before sending to worker
        if patterns.is_empty() {
            return Err(JsValue::from_str("No patterns provided"));
        }

        self.stop_flag.store(false, Ordering::Relaxed);
        self.worker.post_message(&JsValue::from_serde(&patterns)?)?;
        self.is_initialized = true;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn stop_generation(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    #[wasm_bindgen]
    pub fn on_message(&self, callback: js_sys::Function) -> Result<(), JsValue> {
        let closure = Closure::wrap(Box::new(move |event: MessageEvent| {
            callback.call1(&JsValue::NULL, &event.data()).unwrap();
        }) as Box<dyn FnMut(MessageEvent)>);
        
        self.worker.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        Ok(())
    }
} 