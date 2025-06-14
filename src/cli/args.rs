use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The pattern to match in the public key. Use /regex/ for regex patterns, otherwise matches suffix.
    pub pattern: String,

    /// Number of threads to use (defaults to number of CPU cores)
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// ntfy.sh topic to notify when key is found
    #[arg(long)]
    pub ntfy: Option<String>,
}
