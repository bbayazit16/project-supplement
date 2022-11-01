use crate::utils::uint256::Uint256;

pub struct Memory(Vec<char>);

impl Memory {
    pub fn new() -> Self {
        Memory(vec![])
    }

    pub fn store(&mut self, pos: Uint256, val: Uint256) {
        let hex_str = format!("{:0>64}", val.to_hex_str());
        let pos_size = pos.to_uint() as usize;
        if self.0.len() == 0 || self.0.len() < pos_size as usize {
            let rq = pos_size - self.0.len() + 1;
            self.0.extend("0".repeat(rq * 64).chars());
        }
        self.0.splice(pos_size as usize..64, hex_str.chars());
    }

    pub fn load(&self, pos: Uint256) -> Uint256 {
        let i = pos.to_uint() as usize;
        let j = &self.0[i..i + 64];
        let s: String = j.iter().collect();
        Uint256::from_hex(&s)
    }

    pub fn load_var(&self, pos: Uint256, var: Uint256) -> String {
        let i = pos.to_uint() as usize;
        let j = &self.0[i..i + 2 * var.to_uint() as usize];
        j.iter().collect()
    }

    pub fn len(&self) -> Uint256 {
        Uint256::from_int(self.0.len() as i32)
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.len().to_uint() == 0 {
            return write!(f, "{}", "NULL");
        }
        let mut bdr = String::new();
        for i in 0..=self.len().to_uint() as usize / 64 {
            let v = &self.0;
            let j = &v[i * 64..(i + 1) * 64];
            let itr: String = j.iter().collect();
            let fm = &format!("[{i}]: {itr}\n");
            bdr.push_str(fm);
        }
        write!(f, "{}", bdr)
    }
}
