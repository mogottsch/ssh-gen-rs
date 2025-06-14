use clap::Parser;
use num_cpus;
use ssh_gen_rs::{KeyPair, generate_keypair, public_key_ends_with_suffix, save_keypair_to_files};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

use num_format::{Locale, ToFormattedString};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The suffix to search for in the public key
    suffix: String,

    /// Number of threads to use (defaults to number of CPU cores)
    #[arg(short, long)]
    threads: Option<usize>,
}

struct WorkerMessage {
    attempts: u64,
    found_key: Option<KeyPair>,
}

struct SearchResult {
    key_pair: KeyPair,
    total_attempts: u64,
    duration: std::time::Duration,
}

fn generate_and_check_key(suffix: &str) -> (KeyPair, bool) {
    let key_pair = generate_keypair();
    let matches = public_key_ends_with_suffix(&key_pair.public_key, suffix);
    (key_pair, matches)
}

fn send_progress_update(tx: &std::sync::mpsc::Sender<WorkerMessage>, attempts: u64) {
    tx.send(WorkerMessage {
        attempts,
        found_key: None,
    })
    .unwrap();
}

fn send_success(tx: &std::sync::mpsc::Sender<WorkerMessage>, key_pair: KeyPair, attempts: u64) {
    tx.send(WorkerMessage {
        attempts,
        found_key: Some(key_pair),
    })
    .unwrap();
}

fn run_worker_loop(suffix: Arc<String>, tx: std::sync::mpsc::Sender<WorkerMessage>) {
    let mut local_attempts = 0;

    loop {
        let (key_pair, matches) = generate_and_check_key(&suffix);
        local_attempts += 1;

        if matches {
            send_success(&tx, key_pair, local_attempts);
            break;
        }

        if local_attempts % 10000 == 0 {
            send_progress_update(&tx, local_attempts);
            local_attempts = 0;
        }
    }
}

fn spawn_worker_threads(
    n_threads: usize,
    suffix: Arc<String>,
    tx: std::sync::mpsc::Sender<WorkerMessage>,
) -> Vec<thread::JoinHandle<()>> {
    (0..n_threads)
        .map(|_| {
            let tx = tx.clone();
            let suffix = Arc::clone(&suffix);
            thread::spawn(move || run_worker_loop(suffix, tx))
        })
        .collect()
}

fn monitor_progress(rx: std::sync::mpsc::Receiver<WorkerMessage>, start: Instant) -> SearchResult {
    let mut total_attempts = 0;
    let mut found_key_pair = None;

    while found_key_pair.is_none() {
        if let Ok(msg) = rx.recv() {
            total_attempts += msg.attempts;
            if let Some(key) = msg.found_key {
                found_key_pair = Some(key);
            } else if total_attempts % 10000 == 0 {
                let duration = start.elapsed();
                let rate = (total_attempts as f64 / duration.as_secs_f64()).round() as u64;
                println!(
                    "Attempts: {}, Rate: {} keys/sec",
                    total_attempts.to_formatted_string(&Locale::en),
                    rate.to_formatted_string(&Locale::en)
                );
            }
        }
    }

    SearchResult {
        key_pair: found_key_pair.unwrap(),
        total_attempts,
        duration: start.elapsed(),
    }
}

fn print_results(result: &SearchResult, suffix: &str) {
    println!(
        "Found matching key after {} attempts!",
        result.total_attempts
    );
    println!("Time taken: {:.2} seconds", result.duration.as_secs_f64());
    println!(
        "Rate: {:.2} keys/sec",
        result.total_attempts as f64 / result.duration.as_secs_f64()
    );

    if let Err(e) = save_keypair_to_files(&result.key_pair, suffix) {
        println!("Error saving keys: {}", e);
    } else {
        println!("Keys saved to out/{} and out/{}.pub", suffix, suffix);
    }
}

fn find_matching_key(suffix: String, n_threads: usize) -> SearchResult {
    let suffix = Arc::new(suffix);
    let start = Instant::now();
    let (tx, rx) = channel();

    println!("Using {} threads for parallel processing.", n_threads);

    let _handles = spawn_worker_threads(n_threads, Arc::clone(&suffix), tx);
    monitor_progress(rx, start)
}

fn main() {
    let args = Args::parse();
    let n_threads = args.threads.unwrap_or_else(num_cpus::get);

    let result = find_matching_key(args.suffix.clone(), n_threads);
    print_results(&result, &args.suffix);
}
