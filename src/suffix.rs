use ed25519_dalek::VerifyingKey;
use ssh_key::private::Ed25519Keypair;

pub fn public_key_ends_with_suffix(public_key: &VerifyingKey, suffix: &str) -> bool {
    let openssh_pubkey = create_openssh_public_key_from_keypair(public_key);
    let openssh_pubkey_str = openssh_pubkey.to_string();
    let base64_part = extract_base64_from_openssh_string(&openssh_pubkey_str);
    base64_part.ends_with(suffix)
}

fn create_openssh_public_key_from_keypair(
    verifying_key: &VerifyingKey,
) -> ssh_key::public::PublicKey {
    let mut key_bytes = [0u8; 64];
    key_bytes[32..].copy_from_slice(&verifying_key.to_bytes());

    let ed25519_keypair = Ed25519Keypair::from_bytes(&key_bytes).unwrap();
    let openssh_pub = ssh_key::public::Ed25519PublicKey::from(&ed25519_keypair);
    ssh_key::public::PublicKey::from(openssh_pub)
}

fn extract_base64_from_openssh_string(openssh_string: &str) -> &str {
    openssh_string.split_whitespace().nth(1).unwrap_or("")
} 