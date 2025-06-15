use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use ssh_key::{private::Ed25519Keypair, public::Ed25519PublicKey, PrivateKey, PublicKey};

use crate::pattern::Pattern;

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
}

impl KeyPair {
    pub fn public_key_string(&self) -> String {
        let ed25519_keypair = Self::to_ssh_keypair(self);
        let openssh_pub = Ed25519PublicKey::from(&ed25519_keypair);
        let openssh_pubkey = PublicKey::from(openssh_pub);

        openssh_pubkey.to_openssh().unwrap()
    }

    pub fn public_key_string_base64_part(&self) -> String {
        self.public_key_string()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string()
    }

    pub fn private_key_string(&self) -> String {
        let ed25519_keypair = Self::to_ssh_keypair(self);
        let private_key = PrivateKey::from(ed25519_keypair);

        private_key
            .to_openssh(ssh_key::LineEnding::LF)
            .unwrap()
            .to_string()
    }

    pub fn to_ssh_keypair(keypair: &KeyPair) -> Ed25519Keypair {
        let mut key_bytes = [0u8; 64];
        key_bytes[..32].copy_from_slice(&keypair.private_key.to_bytes());
        key_bytes[32..].copy_from_slice(&keypair.public_key.to_bytes());
        Ed25519Keypair::from_bytes(&key_bytes).unwrap()
    }
}

pub fn generate_keypair() -> KeyPair {
    let mut csprng = OsRng;
    let private_key = SigningKey::generate(&mut csprng);
    let public_key = private_key.verifying_key();

    KeyPair {
        public_key,
        private_key,
    }
}

pub fn generate_and_check_key(patterns: &[Pattern]) -> Option<(KeyPair, Pattern)> {
    let keypair = generate_keypair();
    let public_key_base64_part = keypair.public_key_string_base64_part();

    let pattern = patterns
        .iter()
        .find(|p| p.matches_string(&public_key_base64_part));

    if let Some(pattern) = pattern {
        return Some((keypair, pattern.clone()));
    }

    None
}
