use crate::core::keypair::{KeyPair, generate_keypair};
use crate::core::suffix::{Pattern, public_key_matches_pattern};

pub fn generate_and_check_key(pattern: &Pattern) -> (KeyPair, bool) {
    let keypair = generate_keypair();
    let matches = public_key_matches_pattern(&keypair.public_key, pattern);
    (keypair, matches)
}
