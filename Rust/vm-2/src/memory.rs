use super::utils::bit_size;

pub struct Memory(Vec<char>);

impl Memory {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn store(&mut self, idx: u128, v: u128) {
        // self.0.get()
    } 
}