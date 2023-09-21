use log::debug;
use ethnum::u256;

/// Iterator wrapper for u256
pub struct U256Iter{
    value: u256,
}

impl U256Iter{
    /// Create u256 iterator (starts from 1)
    pub fn new() -> Self {
        Self { value: u256::from(0_u32) }
    }
}

impl Iterator for U256Iter {
    type Item = u256;
    fn next(&mut self) -> Option<Self::Item> {
        let new_value = self.value.checked_add(u256::from(1_u32));
        if let Some(new_value) = new_value {
            debug!("Next value: {new_value}");
            self.value  = new_value;
            Some(new_value)
        }
        else{
            debug!("Values are over");
            None
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn iter_test(){
        let mut it = U256Iter::new();
        for i in 1..10_u32{
            assert_eq!(u256::from(i), it.next().unwrap());
        }

        let mut it = U256Iter{value: u256::MAX};
        assert_eq!(None, it.next());
    }
}
