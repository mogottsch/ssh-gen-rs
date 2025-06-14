use crate::core::keypair::KeyPair;

pub struct WorkerMessage {
    pub attempts: u64,
    pub found_key: Option<KeyPair>,
}
