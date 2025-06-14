use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use crate::core::pattern::Pattern;
use crate::core::result::SearchResult;
use crate::worker::message::WorkerMessage;

pub fn monitor_progress(
    rx: Receiver<WorkerMessage>,
    start: Instant,
    pattern: &Pattern,
) -> SearchResult {
    let mut total_attempts = 0;
    let mut found_key_pair = None;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}]\n{msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    while found_key_pair.is_none() {
        if let Ok(msg) = rx.recv() {
            total_attempts += msg.attempts;
            if let Some(key) = msg.found_key {
                found_key_pair = Some(key);
            } else {
                let duration = start.elapsed();
                let rate = (total_attempts as f64 / duration.as_secs_f64()).round() as u64;

                let mut progress_msg = format!(
                    "Attempts: {} ({} keys/sec)",
                    total_attempts.to_formatted_string(&Locale::en),
                    rate.to_formatted_string(&Locale::en)
                );

                if let Some(prob) = pattern.probability() {
                    let expected_attempts = (1.0 / prob) as u64;
                    let est_time = pattern
                        .estimate_time(rate as f64)
                        .unwrap_or_default()
                        .split_whitespace()
                        .take(2)
                        .collect::<Vec<_>>()
                        .join(" ");

                    progress_msg = format!(
                        "{}\nProbability: 1 in {}\nEst. time: {}",
                        progress_msg,
                        expected_attempts.to_formatted_string(&Locale::en),
                        est_time
                    );
                }

                pb.set_message(progress_msg);
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
