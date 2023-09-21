use clap::{Parser, arg, command};
use ethnum::U256;

/// CLI interface for the program
#[derive(Parser)]
#[command(name = "Hash searcher")]
#[command(about = "Searches hashes with trailing zeroes")]
pub struct Cli {
    /// Number of trailing zeroes
    #[arg(
        short = 'N',
        long = "zero",
        default_value = "1"
    )]
    pub zero_count: Option<usize>,
    
    /// Number of hashes to find
    #[arg(
        short = 'F',
        long = "hash",
        default_value = "1"
    )]
    pub hash_count: Option<U256>,

    ///Number of threads to use for hashing
    #[arg(
        short = 'T',
        long = "thread",
        default_value = "8"
    )]
    pub thread_count: Option<usize>,
}

