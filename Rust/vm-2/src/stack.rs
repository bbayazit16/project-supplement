pub struct Stack(Vec<u128>);

impl Stack {
    pub fn push(&mut self, e: u128) {
        self.0.push(e);
    }

    pub fn pop(&mut self) -> Option<u128> {
        self.0.pop()
    }

    pub fn swap(&mut self, a: u128, b: u128) {
        self.0.swap(a as usize, b as usize);
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
