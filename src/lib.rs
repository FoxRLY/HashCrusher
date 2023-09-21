//! Helper tools for creating hash crushing multithreaded program that searches for certain number
//! of hashes with certain number of zeroes on the end.
//! Uses [clap] for CLI interface and [ethnum] for u256 type

/// CLI interface module
pub mod args;

/// U256 Iterator module
pub mod u256;

/// Hash searcher module
pub mod hash_crusher;
