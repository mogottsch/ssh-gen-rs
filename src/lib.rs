use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use ssh_key::LineEnding;
use ssh_key::private::{Ed25519Keypair, PrivateKey};
use std::convert::TryFrom;

pub struct KeyPair {
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
}

pub fn generate_ed25519_keypair() -> KeyPair {
    let mut csprng = OsRng;
    let private_key = SigningKey::generate(&mut csprng);
    let public_key = private_key.verifying_key();

    KeyPair {
        public_key,
        private_key,
    }
}

fn convert_to_ssh_public_key_bytes(verifying_key: &VerifyingKey) -> [u8; 32] {
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&verifying_key.to_bytes());
    key_bytes
}

fn create_openssh_public_key_from_bytes(key_bytes: &[u8]) -> ssh_key::public::PublicKey {
    let openssh_pub = ssh_key::public::Ed25519PublicKey::try_from(key_bytes).unwrap();
    ssh_key::public::PublicKey::from(openssh_pub)
}

fn extract_base64_from_openssh_string(openssh_string: &str) -> &str {
    openssh_string.split_whitespace().nth(1).unwrap_or("")
}

pub fn public_key_ends_with_suffix(public_key: &VerifyingKey, suffix: &str) -> bool {
    let key_bytes = convert_to_ssh_public_key_bytes(public_key);
    let openssh_pubkey = create_openssh_public_key_from_bytes(&key_bytes);
    let openssh_pubkey_str = openssh_pubkey.to_string();
    let base64_part = extract_base64_from_openssh_string(&openssh_pubkey_str);
    base64_part.ends_with(suffix)
}

fn create_ssh_keypair_from_ed25519_keys(keypair: &KeyPair) -> Ed25519Keypair {
    let mut key_bytes = [0u8; 64];
    key_bytes[..32].copy_from_slice(&keypair.private_key.to_bytes());
    key_bytes[32..].copy_from_slice(&keypair.public_key.to_bytes());
    Ed25519Keypair::from_bytes(&key_bytes).unwrap()
}

fn write_public_key_to_file(private_key: &PrivateKey) -> std::io::Result<()> {
    let public_key = private_key.public_key();
    std::fs::write("id_ed25519.pub", public_key.to_string())
}

fn write_private_key_to_file(private_key: &PrivateKey) -> std::io::Result<()> {
    let pem = private_key.to_openssh(LineEnding::LF).unwrap();
    std::fs::write("id_ed25519", pem.as_str())
}

pub fn save_keypair_to_files(keypair: &KeyPair) -> std::io::Result<()> {
    let ed25519_keypair = create_ssh_keypair_from_ed25519_keys(keypair);
    let private_key = PrivateKey::from(ed25519_keypair);

    write_public_key_to_file(&private_key)?;
    write_private_key_to_file(&private_key)?;

    Ok(())
}

