use crate::core::file_io::save_keypair_to_files;
use crate::monitor::result::SearchResult;
use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};

pub fn print_results(result: &SearchResult, suffix: &str) {
    let pb = ProgressBar::new(1);
    pb.set_style(ProgressStyle::default_bar().template("{msg}").unwrap());

    pb.println(format!(
        "✨ Found matching key after {} attempts!",
        result.total_attempts.to_formatted_string(&Locale::en)
    ));
    pb.println(format!(
        "⏱️  Time taken: {:.2} seconds",
        result.duration.as_secs_f64()
    ));
    pb.println(format!(
        "🚀 Rate: {:.2} keys/sec",
        result.total_attempts as f64 / result.duration.as_secs_f64()
    ));

    if let Err(e) = save_keypair_to_files(&result.key_pair, suffix) {
        pb.println(format!("❌ Error saving keys: {}", e));
    } else {
        pb.println(format!(
            "💾 Keys saved to out/{} and out/{}.pub",
            suffix, suffix
        ));
    }

    pb.finish_and_clear();
}
