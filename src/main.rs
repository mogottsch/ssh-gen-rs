use clap::Parser;
use num_cpus;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Instant;

mod cli;
mod core;
mod monitor;
mod output;
mod worker;

use cli::Args;
use monitor::monitor_progress;
use output::print_results;
use worker::spawn_worker_threads;

fn main() {
    let args = Args::parse();
    let n_threads = args.threads.unwrap_or_else(num_cpus::get);

    let result = find_matching_key(args.suffix.clone(), n_threads);
    print_results(&result, &args.suffix);
}

fn find_matching_key(suffix: String, n_threads: usize) -> monitor::SearchResult {
    let suffix = Arc::new(suffix);
    let start = Instant::now();
    let (tx, rx) = channel();

    println!("Using {} threads for parallel processing.", n_threads);

    let _handles = spawn_worker_threads(n_threads, Arc::clone(&suffix), tx);
    monitor_progress(rx, start)
}
