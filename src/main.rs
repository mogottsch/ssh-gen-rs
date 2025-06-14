use ssh_gen_rs::{generate_keypair, public_key_ends_with_suffix, save_keypair_to_files};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <suffix>", args[0]);
        println!("Example: {} ye", args[0]);
        return;
    }

    let suffix = &args[1];
    let mut attempts = 0;
    let start = Instant::now();

    loop {
        let key_pair = generate_keypair();
        attempts += 1;

        if public_key_ends_with_suffix(&key_pair.public_key, suffix) {
            let duration = start.elapsed();
            println!("Found matching key after {} attempts!", attempts);
            println!("Time taken: {:.2} seconds", duration.as_secs_f64());
            println!(
                "Rate: {:.2} keys/sec",
                attempts as f64 / duration.as_secs_f64()
            );

            if let Err(e) = save_keypair_to_files(&key_pair) {
                println!("Error saving keys: {}", e);
            } else {
                println!("Keys saved to id_ed25519 and id_ed25519.pub");
            }
            break;
        }

        if attempts % 1000 == 0 {
            let duration = start.elapsed();
            let rate = attempts as f64 / duration.as_secs_f64();
            println!("Attempts: {}, Rate: {:.2} keys/sec", attempts, rate);
        }
    }
}
