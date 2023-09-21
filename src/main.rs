use std::process::exit;
use ethnum::U256;
use hashcrusher::{args::Cli, hash_crusher::get_hashes};
use clap::Parser;
use log::info;

fn main(){
    env_logger::init();
    let a = Cli::parse();
    let zero_count = a.zero_count.unwrap_or(1);
    let thread_count = a.thread_count.unwrap_or(1);
    if thread_count < 1 {
        println!("Error: thread count cannot be less than 1");
        exit(1);
    }
    let hash_count = a.hash_count.unwrap_or(U256::from(1_u32));
    
    info!("Mode:\ntrailing_zeroes: {zero_count}\nthread_count: {thread_count}\nhash_count: {hash_count}");
    
    info!("Started searching hashes...");
    let hashes = get_hashes(hash_count, zero_count, thread_count);
    info!("Search is over");
    for i in hashes.iter(){
        println!("{}, {}", i.0, i.1);
    }
}

