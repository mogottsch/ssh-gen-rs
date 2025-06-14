use num_cpus;
use rayon::prelude::*;
use ssh_gen_rs::{generate_keypair, public_key_ends_with_suffix, save_keypair_to_files};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <suffix>", args[0]);
        println!("Example: {} ye", args[0]);
        return;
    }

    let suffix = Arc::new(args[1].clone());
    let attempts = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let n_cpus = num_cpus::get();
    println!("Using {} CPU cores for parallel processing.", n_cpus);

    let found_key_pair = (0..n_cpus).into_par_iter().find_map_any(|_| {
        loop {
            let key_pair = generate_keypair();
            let current_attempts = attempts.fetch_add(1, Ordering::SeqCst);

            if public_key_ends_with_suffix(&key_pair.public_key, &suffix) {
                return Some(key_pair);
            }

            if current_attempts % 100000 == 0 && current_attempts > 0 {
                let duration = start.elapsed();
                let rate = current_attempts as f64 / duration.as_secs_f64();
                println!("Attempts: {}, Rate: {:.2} keys/sec", current_attempts, rate);
            }
        }
    });

    if let Some(key_pair) = found_key_pair {
        let total_attempts = attempts.load(Ordering::SeqCst);
        let duration = start.elapsed();
        println!("Found matching key after {} attempts!", total_attempts);
        println!("Time taken: {:.2} seconds", duration.as_secs_f64());
        println!(
            "Rate: {:.2} keys/sec",
            total_attempts as f64 / duration.as_secs_f64()
        );

        if let Err(e) = save_keypair_to_files(&key_pair, &suffix) {
            println!("Error saving keys: {}", e);
        } else {
            println!("Keys saved to out/{} and out/{}.pub", &suffix, &suffix);
        }
    }
}
