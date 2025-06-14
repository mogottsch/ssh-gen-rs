use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use crate::cli::Args;
use crate::core::file_io::save_keypair_to_files;
use crate::core::keypair::KeyPair;
use crate::core::pattern::Pattern;
use crate::worker::message::WorkerMessage;

use super::ntfy::notify;

pub fn run_manager(rx: Receiver<WorkerMessage>, start: Instant, patterns: &[Pattern], args: &Args) {
    let mut total_attempts = 0;
    let mut pattern_key_pairs: HashMap<Pattern, Vec<KeyPair>> = HashMap::new();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}]\n{msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    loop {
        if let Ok(msg) = rx.recv() {
            total_attempts += msg.attempts;

            let duration = start.elapsed();
            let rate = (total_attempts as f64 / duration.as_secs_f64()).round() as u64;

            let mut progress_msg = format!(
                "Attempts: {} ({} keys/sec)",
                total_attempts.to_formatted_string(&Locale::en),
                rate.to_formatted_string(&Locale::en)
            );

            // Show estimates for each pattern
            for pattern in patterns.iter() {
                let pattern_str = match pattern {
                    Pattern::Suffix(s) => s.as_str(),
                    Pattern::Regex(r) => r.as_str(),
                };

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
                        "{}\nPattern '{}': 1 in {} (est. {})",
                        progress_msg,
                        pattern_str,
                        expected_attempts.to_formatted_string(&Locale::en),
                        est_time
                    );
                } else {
                    progress_msg = format!(
                        "{}\nPattern '{}': regex pattern (no estimate)",
                        progress_msg, pattern_str
                    );
                }

                let n_hits = pattern_key_pairs.get(pattern).map_or(0, |keys| keys.len());

                if n_hits > 0 {
                    let is_plural = if n_hits == 1 { "" } else { "s" };
                    progress_msg = format!(
                        "{} | {} key{} found",
                        progress_msg,
                        n_hits.to_formatted_string(&Locale::en),
                        is_plural
                    );
                }
            }

            pb.set_message(progress_msg);
            if let Some(search_hit) = msg.search_hit {
                let key_pair = search_hit.key_pair;
                let pattern = search_hit.pattern;

                pattern_key_pairs
                    .entry(pattern.clone())
                    .or_default()
                    .push(key_pair.clone());

                let filename = pattern.to_filename();
                pb.println(format!("✨ Found matching key for pattern '{}'", pattern,));
                save_keypair_to_files(&key_pair, &filename)
                    .expect("Failed to save keypair to files");
                pb.println(format!("Key saved to 'out/{}'", filename));

                if let Some(topic) = &args.ntfy {
                    notify(topic, &format!("Found key matching pattern '{}'", pattern,))
                        .expect("Failed to send ntfy notification");
                }

                if args.stop_after_match {
                    pb.finish_and_clear();
                    break;
                }
            }
        }
    }
}
