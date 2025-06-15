use core::{keypair::KeyPair, pattern::Pattern};

pub struct SearchHit {
    pub key_pair: KeyPair,
    pub pattern: Pattern,
}

pub struct WorkerMessage {
    pub attempts: u64,
    pub search_hit: Option<SearchHit>,
}
