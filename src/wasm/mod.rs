use wasm_bindgen::prelude::*;
use crate::core::keypair::KeyPair;
use crate::core::pattern::Pattern;
use ed25519_dalek::pkcs8::EncodePrivateKey;

#[wasm_bindgen]
pub struct JsKeyPair {
    public_key: String,
    private_key: String,
}

#[wasm_bindgen]
impl JsKeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }
    pub fn from_keypair(public_key_str: &str, private_key_str: &str) -> Self {
        JsKeyPair {
            public_key: public_key_str.to_string(),
            private_key: private_key_str.to_string(),
        }
    }
}

#[wasm_bindgen]
pub struct KeyGenerator {
    patterns: Vec<Pattern>,
    attempts: u64,
    is_running: bool,
    last_error: Option<String>,
}

#[wasm_bindgen]
impl KeyGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(patterns: Vec<String>) -> Result<KeyGenerator, JsValue> {
        if patterns.is_empty() {
            return Err(JsValue::from_str("No patterns provided"));
        }
        for pattern in &patterns {
            if pattern.len() > 100 {
                return Err(JsValue::from_str("Pattern too long"));
            }
        }
        Ok(KeyGenerator {
            patterns: patterns.into_iter()
                .map(|p| Pattern::new(p))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
            attempts: 0,
            is_running: false,
            last_error: None,
        })
    }

    #[wasm_bindgen]
    pub fn generate_batch(&mut self, batch_size: u32) -> Option<JsKeyPair> {
        if self.is_running {
            return None;
        }
        if batch_size > 10000 {
            self.last_error = Some("Batch size too large".to_string());
            return None;
        }
        self.is_running = true;
        self.last_error = None;
        let mut found: Option<JsKeyPair> = None;
        for _ in 0..batch_size {
            self.attempts += 1;
            if let Some((key_pair, _)) = crate::worker::generator::generate_and_check_key(&self.patterns) {
                let public_key_str = crate::core::pattern::create_openssh_public_key_from_keypair(&key_pair.public_key).to_string();
                let private_key_str = key_pair.private_key.to_pkcs8_pem(Default::default()).unwrap().to_string();
                found = Some(JsKeyPair::from_keypair(&public_key_str, &private_key_str));
                break;
            }
        }
        self.is_running = false;
        found
    }

    #[wasm_bindgen]
    pub fn get_attempts(&self) -> u64 {
        self.attempts
    }

    #[wasm_bindgen]
    pub fn get_last_error(&self) -> Option<String> {
        self.last_error.clone()
    }
} 