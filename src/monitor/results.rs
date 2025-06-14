use crate::core::file_io::save_keypair_to_files;
use crate::core::pattern::Pattern;
use crate::core::result::SearchResult;
use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};

pub fn print_results(result: &SearchResult, pattern: &str) -> std::io::Result<()> {
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

    let key_pair = &result.key_pair;
    let pattern = Pattern::new(pattern.to_string()).unwrap();
    let filename = pattern.to_filename();
    save_keypair_to_files(key_pair, &filename)?;
    pb.println(format!(
        "💾 Keys saved to out/{} and out/{}.pub",
        filename, filename
    ));

    pb.finish_and_clear();
    Ok(())
}
