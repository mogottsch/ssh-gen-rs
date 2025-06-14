use crate::core::keypair::KeyPair;
use ssh_key::LineEnding;
use ssh_key::private::{Ed25519Keypair, PrivateKey};
use std::fs;
use std::path::Path;

pub fn save_keypair_to_files(keypair: &KeyPair, filename: &str) -> std::io::Result<()> {
    create_out_directory()?;

    let ed25519_keypair = create_ssh_keypair_from_ed25519_keys(keypair);
    let private_key = PrivateKey::from(ed25519_keypair);

    write_public_key_to_file(&private_key, &filename)?;
    write_private_key_to_file(&private_key, &filename)?;

    Ok(())
}

fn create_out_directory() -> std::io::Result<()> {
    if !Path::new("out").exists() {
        fs::create_dir("out")?;
    }
    Ok(())
}

fn create_ssh_keypair_from_ed25519_keys(keypair: &KeyPair) -> Ed25519Keypair {
    let mut key_bytes = [0u8; 64];
    key_bytes[..32].copy_from_slice(&keypair.private_key.to_bytes());
    key_bytes[32..].copy_from_slice(&keypair.public_key.to_bytes());
    Ed25519Keypair::from_bytes(&key_bytes).unwrap()
}

fn write_public_key_to_file(private_key: &PrivateKey, filename: &str) -> std::io::Result<()> {
    let public_key = private_key.public_key();
    let filename = format!("out/{}.pub", filename);
    std::fs::write(filename, public_key.to_string())
}

fn write_private_key_to_file(private_key: &PrivateKey, suffix: &str) -> std::io::Result<()> {
    let pem = private_key.to_openssh(LineEnding::LF).unwrap();
    let filename = format!("out/{}", suffix);
    std::fs::write(filename, pem.as_str())
}
