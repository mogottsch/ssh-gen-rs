use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The suffix to search for in the public key
    pub suffix: String,

    /// Number of threads to use (defaults to number of CPU cores)
    #[arg(short, long)]
    pub threads: Option<usize>,
}
