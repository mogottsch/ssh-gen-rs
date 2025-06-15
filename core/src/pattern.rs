use regex::Regex;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::keypair::KeyPair;

#[derive(Debug, Clone)]
pub enum Pattern {
    Suffix(String),
    Regex(Regex),
}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Pattern::Suffix(suffix) => suffix.hash(state),
            Pattern::Regex(regex) => regex.as_str().hash(state),
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pattern::Suffix(s1), Pattern::Suffix(s2)) => s1 == s2,
            (Pattern::Regex(r1), Pattern::Regex(r2)) => r1.as_str() == r2.as_str(),
            _ => false,
        }
    }
}

impl Eq for Pattern {}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Suffix(suffix) => write!(f, "Suffix: {}", suffix),
            Pattern::Regex(regex) => write!(f, "Regex: {}", regex.as_str()),
        }
    }
}

impl Pattern {
    pub fn new(pattern: String) -> Result<Self, regex::Error> {
        if pattern.starts_with('/') && pattern.ends_with('/') {
            let pattern = pattern[1..pattern.len() - 1].to_string();
            Ok(Pattern::Regex(Regex::new(&pattern)?))
        } else {
            Ok(Pattern::Suffix(pattern))
        }
    }

    pub fn to_filename(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        match self {
            Pattern::Suffix(suffix) => format!("{}_{}", suffix, timestamp),
            Pattern::Regex(regex) => {
                let pattern = regex.as_str();
                // Remove special characters and limit length
                let clean = pattern
                    .chars()
                    .take(20)
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect::<String>();
                format!("regex_{}_{}", clean, timestamp)
            }
        }
    }

    pub fn probability(&self) -> Option<f64> {
        match self {
            Pattern::Suffix(suffix) => {
                // Base64 has 64 possible characters
                let base: f64 = 64.0;
                // Probability is (1/64)^n where n is the length of the suffix
                Some(1.0 / base.powi(suffix.len() as i32))
            }
            Pattern::Regex(_) => None, // Regex patterns are too complex to calculate probability
        }
    }

    pub fn estimate_time(&self, keys_per_second: f64) -> Option<String> {
        self.probability().map(|prob| {
            let expected_attempts = 1.0 / prob;
            let seconds = expected_attempts / keys_per_second;
            if seconds > u64::MAX as f64 {
                return "∞".to_string();
            }
            let duration = Duration::from_secs_f64(seconds);
            humantime::format_duration(duration).to_string()
        })
    }

    pub fn matches_keypair(&self, keypair: &KeyPair) -> bool {
        let base64_part = keypair.public_key_string_base64_part();

        self.matches_string(&base64_part)
    }

    pub fn matches_string(&self, base64_part: &str) -> bool {
        match self {
            Pattern::Suffix(suffix) => base64_part.ends_with(suffix),
            Pattern::Regex(regex) => regex.is_match(base64_part),
        }
    }
}
