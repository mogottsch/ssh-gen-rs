use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct KeyPair {
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
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
