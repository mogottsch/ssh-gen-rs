use clap::Parser;
use num_cpus;
use rayon::prelude::*;
use ssh_gen_rs::{KeyPair, generate_keypair, public_key_ends_with_suffix, save_keypair_to_files};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The suffix to search for in the public key
    suffix: String,

    /// Number of threads to use (defaults to number of CPU cores)
    #[arg(short, long)]
    threads: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let suffix = Arc::new(args.suffix);
    let attempts = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let n_threads = args.threads.unwrap_or_else(num_cpus::get);
    println!("Using {} threads for parallel processing.", n_threads);

    let found_key_pair = (0..n_threads).into_par_iter().find_map_any(|_| {
        loop {
            let key_pair = generate_keypair();
            let current_attempts = attempts.fetch_add(1, Ordering::SeqCst);

            if public_key_ends_with_suffix(&key_pair.public_key, &suffix) {
                return Some(key_pair);
            }

            if current_attempts % 10000 == 0 && current_attempts > 0 {
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
