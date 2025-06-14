use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use std::sync::mpsc::Receiver;
use std::time::Instant;

use crate::monitor::result::SearchResult;
use crate::worker::message::WorkerMessage;

pub fn monitor_progress(rx: Receiver<WorkerMessage>, start: Instant) -> SearchResult {
    let mut total_attempts = 0;
    let mut found_key_pair = None;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );

    while found_key_pair.is_none() {
        if let Ok(msg) = rx.recv() {
            total_attempts += msg.attempts;
            if let Some(key) = msg.found_key {
                found_key_pair = Some(key);
            } else {
                let duration = start.elapsed();
                let rate = (total_attempts as f64 / duration.as_secs_f64()).round() as u64;
                pb.set_message(format!(
                    "Attempts: {} ({} keys/sec)",
                    total_attempts.to_formatted_string(&Locale::en),
                    rate.to_formatted_string(&Locale::en)
                ));
            }
        }
    }

    pb.finish_and_clear();

    SearchResult {
        key_pair: found_key_pair.unwrap(),
        total_attempts,
        duration: start.elapsed(),
    }
}
