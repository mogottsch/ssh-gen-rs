use crate::core::keypair::{KeyPair, generate_keypair};
use crate::core::pattern::{Pattern, public_key_matches_pattern};

pub fn generate_and_check_key(patterns: &[Pattern]) -> Option<(KeyPair, Pattern)> {
    let keypair = generate_keypair();
    let pattern = patterns
        .iter()
        .find(|p| public_key_matches_pattern(&keypair.public_key, p));

    if let Some(pattern) = pattern {
        return Some((keypair, pattern.clone()));
    }

    None
}
