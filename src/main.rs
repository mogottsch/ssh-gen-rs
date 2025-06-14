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
use core::pattern::Pattern;
use monitor::monitor_progress;
use worker::spawn_worker_threads;

fn main() {
    let args = Args::parse();
    let n_threads = args.threads.unwrap_or_else(num_cpus::get);

    let patterns: Result<Vec<Pattern>, regex::Error> = args
        .patterns
        .iter()
        .map(|p| Pattern::new(p.clone()))
        .collect();

    let patterns = match patterns {
        Ok(patterns) => patterns,
        Err(e) => {
            eprintln!("Invalid pattern: {}", e);
            std::process::exit(1);
        }
    };

    find_matching_key(patterns, n_threads, args);
}

fn find_matching_key(patterns: Vec<Pattern>, n_threads: usize, args: Args) {
    let patterns = Arc::new(patterns);
    let start = Instant::now();
    let (tx, rx) = channel();
    let stop_flag = Arc::new(AtomicBool::new(false));

    println!("Using {} threads for parallel processing.", n_threads);

    let _handles =
        spawn_worker_threads(n_threads, Arc::clone(&patterns), tx, Arc::clone(&stop_flag));

    monitor_progress(rx, start, &patterns, &args);
    stop_flag.store(true, Ordering::Relaxed);
}
