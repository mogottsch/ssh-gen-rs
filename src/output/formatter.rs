use crate::core::file_io::save_keypair_to_files;
use crate::monitor::result::SearchResult;
use num_format::{Locale, ToFormattedString};

pub fn print_results(result: &SearchResult, suffix: &str) {
    println!(
        "Found matching key after {} attempts!",
        result.total_attempts.to_formatted_string(&Locale::en)
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
