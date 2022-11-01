use crate::utils::uint256::Uint256;

pub struct Bytecode(Vec<Uint256>);

impl Bytecode {
    pub fn from_hex_str(b: &str) -> Self {
        let mut v: Vec<Uint256> = Vec::new();
        let mut i: usize = 0;

        while i < b.len() - 1 {
            let tok = b[i..i + 2].parse::<u8>().unwrap();

            v.push(Uint256::from_int(tok as i32));

            if tok < 32 {
                let push_size = (tok + 1) * 2;
                let push_tok = &b[i + 2..i + 2 + push_size as usize];
                v.push(Uint256::from_hex(push_tok));
                i += push_size as usize + 2;
                continue;
            }

            i += 2;
        }

        Bytecode(v)
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    pub fn get(&mut self, i: usize) -> &Uint256 {
        &self.0[i]
    }
}

impl std::fmt::Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
