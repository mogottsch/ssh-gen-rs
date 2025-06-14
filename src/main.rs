use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use ssh_key::LineEnding;
use ssh_key::private::{Ed25519Keypair, PrivateKey};
use std::convert::TryFrom;
use std::time::Instant;

struct KeyPair {
    public_key: VerifyingKey,
    private_key: SigningKey,
}

fn generate_key() -> KeyPair {
    let mut csprng = OsRng;
    let private_key = SigningKey::generate(&mut csprng);
    let public_key = private_key.verifying_key();

    KeyPair {
        public_key,
        private_key,
    }
}

fn check_suffix(public_key: &VerifyingKey, suffix: &str) -> bool {
    // Build OpenSSH public key string as it will appear in the .pub file
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&public_key.to_bytes());
    let openssh_pub = ssh_key::public::Ed25519PublicKey::try_from(&key_bytes[..]).unwrap();
    let openssh_pubkey = ssh_key::public::PublicKey::from(openssh_pub);
    let openssh_pubkey_str = openssh_pubkey.to_string();
    let b64 = openssh_pubkey_str.split_whitespace().nth(1).unwrap_or("");
    b64.ends_with(suffix)
}

fn save_key_pair(key_pair: &KeyPair) -> std::io::Result<()> {
    // Convert ed25519-dalek keys to ssh-key format
    let mut key_bytes = [0u8; 64];
    key_bytes[..32].copy_from_slice(&key_pair.private_key.to_bytes());
    key_bytes[32..].copy_from_slice(&key_pair.public_key.to_bytes());
    let ed25519_keypair = Ed25519Keypair::from_bytes(&key_bytes).unwrap();
    let private_key = PrivateKey::from(ed25519_keypair);

    // Save public key in OpenSSH format
    let public_key = private_key.public_key();
    println!("save_key_pair: public key: {}", public_key.to_string());
    std::fs::write("id_ed25519.pub", public_key.to_string())?;

    // Save private key in OpenSSH format (PEM)
    let pem = private_key.to_openssh(LineEnding::LF).unwrap();
    println!("save_key_pair: private key PEM: {:?}", pem);
    std::fs::write("id_ed25519", pem.as_str())?;

    Ok(())
}

fn main() {
    let suffix = "ye"; // We'll make this configurable later
    let mut attempts = 0;
    let start = Instant::now();

    loop {
        let key_pair = generate_key();
        attempts += 1;

        if check_suffix(&key_pair.public_key, suffix) {
            continue;
            // let duration = start.elapsed();
            // println!("Found matching key after {} attempts!", attempts);
            // println!("Time taken: {:.2} seconds", duration.as_secs_f64());
            // println!(
            //     "Rate: {:.2} keys/sec",
            //     attempts as f64 / duration.as_secs_f64()
            // );
            // println!(
            //     "Public key: {}",
            //     BASE64.encode(key_pair.public_key.to_bytes())
            // );
            //
            // if let Err(e) = save_key_pair(&key_pair) {
            //     println!("Error saving keys: {}", e);
            // } else {
            //     println!("Keys saved to id_ed25519 and id_ed25519.pub");
            // }
            // break;
        }

        if attempts % 1000 == 0 {
            let duration = start.elapsed();
            let rate = attempts as f64 / duration.as_secs_f64();
            println!("Attempts: {}, Rate: {:.2} keys/sec", attempts, rate);
        }

        if attempts == 10000 {
            break;
        }
    }
}
