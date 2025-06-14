use crate::core::keypair::KeyPair;
use std::time::Duration;

pub struct SearchResult {
    pub key_pair: KeyPair,
    pub total_attempts: u64,
    pub duration: Duration,
}
