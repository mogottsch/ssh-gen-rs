use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::worker::generator::generate_and_check_key;
use crate::worker::message::WorkerMessage;
use crate::core::keypair::KeyPair;
use crate::core::suffix::public_key_ends_with_suffix;
use crate::core::keypair::generate_keypair;

pub fn spawn_worker_threads(
    n_threads: usize,
    suffix: Arc<String>,
    tx: Sender<WorkerMessage>,
    stop_flag: Arc<AtomicBool>,
) -> Vec<thread::JoinHandle<()>> {
    (0..n_threads)
        .map(|_| {
            let tx = tx.clone();
            let suffix = Arc::clone(&suffix);
            let stop_flag = Arc::clone(&stop_flag);
            thread::spawn(move || run_worker_loop(suffix, tx, stop_flag))
        })
        .collect()
}

pub fn run_worker_loop(suffix: Arc<String>, tx: Sender<WorkerMessage>, stop_flag: Arc<AtomicBool>) {
    let mut local_attempts = 0;

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
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

pub fn send_success(
    tx: &Sender<WorkerMessage>,
    key_pair: crate::core::keypair::KeyPair,
    attempts: u64,
) {
    tx.send(WorkerMessage {
        attempts,
        found_key: Some(key_pair),
    })
    .unwrap();
}

pub fn send_progress_update(tx: &Sender<WorkerMessage>, attempts: u64) {
    tx.send(WorkerMessage {
        attempts,
        found_key: None,
    })
    .unwrap();
}

pub fn worker_thread(suffix: Arc<String>, tx: Sender<KeyPair>, stop_flag: Arc<AtomicBool>) {
    loop {
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
        let keypair = generate_keypair();
        if public_key_ends_with_suffix(&keypair.public_key, &suffix) {
            if tx.send(keypair).is_err() {
                break;
            }
        }
    }
}
