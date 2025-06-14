use crate::core::keypair::{KeyPair, generate_keypair};
use crate::core::suffix::public_key_ends_with_suffix;

pub fn generate_and_check_key(suffix: &str) -> (KeyPair, bool) {
    let key_pair = generate_keypair();
    let matches = public_key_ends_with_suffix(&key_pair.public_key, suffix);
    (key_pair, matches)
}
