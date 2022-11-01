use crate::utils::uint256::Uint256;

pub struct Stack(Vec<Uint256>);

impl Stack {
    pub fn new() -> Self {
        Stack(vec![])
    }

    pub fn push(&mut self, n: Uint256) {
        self.0.push(n);
    }

    pub fn pop(&mut self) -> Uint256 {
        self.0.pop().unwrap()
    }

    pub fn swap(&mut self, a: Uint256, b: Uint256) {
        self.0.swap(a.to_uint() as usize, b.to_uint() as usize);
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
