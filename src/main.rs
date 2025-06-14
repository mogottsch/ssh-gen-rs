use clap::Parser;
use num_cpus;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::Instant;

mod cli;
mod core;
mod monitor;
mod worker;

use cli::Args;
use core::suffix::Pattern;
use monitor::ntfy;
use monitor::{monitor_progress, print_results};
use worker::spawn_worker_threads;

fn main() {
    let args = Args::parse();
    let n_threads = args.threads.unwrap_or_else(num_cpus::get);

    let pattern = match Pattern::new(args.pattern.clone()) {
        Ok(pattern) => pattern,
        Err(e) => {
            eprintln!("Invalid pattern: {}", e);
            std::process::exit(1);
        }
    };

    let result = find_matching_key(pattern, n_threads);
    print_results(&result, &args.pattern).unwrap_or_else(|e| {
        eprintln!("Error printing results: {}", e);
        std::process::exit(1);
    });

    if let Some(topic) = args.ntfy {
        let message = format!("Found key matching pattern: {}", args.pattern);
        if let Err(e) = ntfy::notify(&topic, &message) {
            eprintln!("Failed to send ntfy notification: {}", e);
        }
    }
}

fn find_matching_key(pattern: Pattern, n_threads: usize) -> monitor::SearchResult {
    let pattern = Arc::new(pattern);
    let start = Instant::now();
    let (tx, rx) = channel();
    let stop_flag = Arc::new(AtomicBool::new(false));

    println!("Using {} threads for parallel processing.", n_threads);

    let _handles =
        spawn_worker_threads(n_threads, Arc::clone(&pattern), tx, Arc::clone(&stop_flag));
    let result = monitor_progress(rx, start);
    stop_flag.store(true, Ordering::Relaxed);
    result
}
