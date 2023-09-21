use ethnum::U256;
use crate::u256::U256Iter;
use std::{sync::{Arc, Mutex}, panic};
use sha256::digest;
use log::{debug, error, info};


/// Searches hashes with trailing zeroes using multithreading
///
/// If thread count is 0, no searching is done, resulting is empty array
/// Panics if something really goes wrong with memory
pub fn get_hashes(hash_count: U256, zero_count: usize, thread_count: usize) -> Vec<(U256, String)> {
    let end_zeroes = Arc::new("0".repeat(zero_count));
    let number = Arc::new(Mutex::new(U256Iter::new()));
    let hashes = Arc::new(Mutex::new(vec![]));
    std::thread::scope(|s|{
        let mut threads = vec![];
        for i in 0..thread_count{
            let number_c = number.clone();
            let hashes_c = hashes.clone();
            let pattern = end_zeroes.clone();
            let thread = s.spawn(move||{
                loop{
                    let new_number = number_c.lock().unwrap_or_else(|_|{
                        error!("U256 iterator mutex acquisition failed, aborting execution");
                        panic!("Internal error, aborting");
                    }).next();
                    if let Some(val) = new_number{
                        info!("Thread {i} started processing value {val}");
                        let new_hash = digest(&val.to_le_bytes());
                        let mut hashes = hashes_c.lock().unwrap_or_else(|_|{
                            error!("Hash array mutex acquisition failed, aborting execution");
                            panic!("Internal error, aborting");
                        });
                        if U256::from(hashes.len() as u32) >= hash_count {
                            break;
                        }
                        if new_hash.ends_with(pattern.as_str()){
                            debug!("Thread {i} pushed new hash {new_hash} into hash array");
                            hashes.push((val, new_hash));
                        }
                        info!("Thread {i} stopped processing value {val}");
                    } else {
                        info!("Thread {i} ended executing");
                        break;
                    }
                }
            });
            threads.push(thread);
        }
    });
    let lock = Arc::try_unwrap(hashes).unwrap_or_else(|_|{
        error!("Thread work result cannot be obtained, aborting execution");
        panic!("Internal error, aborting");
    });
    lock.into_inner().unwrap_or_else(|_|{
        error!("Thread mutex acquisition failed, aborting execution");
        panic!("Internal error, aborting");
    })
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn hash_test(){
        // simple usage
        let (hash_count, zero_count, thread_count) = (U256::from(1_u32),1,1);
        let result = get_hashes(hash_count, zero_count, thread_count);
        assert_eq!(1, result.len());
        for i in result{
            assert!(i.1.ends_with("0"));
        }
        
        // typical usage
        let (hash_count, zero_count, thread_count) = (U256::from(3_u32),3,4);
        let result = get_hashes(hash_count, zero_count, thread_count);
        assert_eq!(3, result.len());
        for i in result{
            assert!(i.1.ends_with("000"));
        }

        // find zero hashes
        let (hash_count, zero_count, thread_count) = (U256::from(0_u32),1,1);
        let result = get_hashes(hash_count, zero_count, thread_count);
        assert_eq!(0, result.len());
        for i in result{
            assert!(i.1.ends_with("0"));
        }

        // find hashes with zero zeroes on the end
        let (hash_count, zero_count, thread_count) = (U256::from(1_u32),0,1);
        let result = get_hashes(hash_count, zero_count, thread_count);
        assert_eq!(1, result.len());
        for i in result{
            assert!(i.1.ends_with(""));
        }

        // find hashes with zero threads must result in empty array as no work is done
        let (hash_count, zero_count, thread_count) = (U256::from(3_u32),3,0);
        let result = get_hashes(hash_count, zero_count, thread_count);
        assert_eq!(0, result.len());
        for i in result{
            assert!(i.1.ends_with("000"));
        }
    }
}
