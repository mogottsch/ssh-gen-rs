use crate::keypair::KeyPair;
use ssh_key::LineEnding;
use ssh_key::private::{Ed25519Keypair, PrivateKey};
use std::fs;
use std::path::Path;

pub fn save_keypair_to_files(keypair: &KeyPair, suffix: &str) -> std::io::Result<()> {
    let safe_suffix = make_suffix_filesystem_safe(suffix);
    create_out_directory()?;

    let ed25519_keypair = create_ssh_keypair_from_ed25519_keys(keypair);
    let private_key = PrivateKey::from(ed25519_keypair);

    write_public_key_to_file(&private_key, &safe_suffix)?;
    write_private_key_to_file(&private_key, &safe_suffix)?;

    Ok(())
}

fn make_suffix_filesystem_safe(suffix: &str) -> String {
    suffix
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
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

fn write_public_key_to_file(private_key: &PrivateKey, suffix: &str) -> std::io::Result<()> {
    let public_key = private_key.public_key();
    let filename = format!("out/{}.pub", suffix);
    std::fs::write(filename, public_key.to_string())
}

fn write_private_key_to_file(private_key: &PrivateKey, suffix: &str) -> std::io::Result<()> {
    let pem = private_key.to_openssh(LineEnding::LF).unwrap();
    let filename = format!("out/{}", suffix);
    std::fs::write(filename, pem.as_str())
}

