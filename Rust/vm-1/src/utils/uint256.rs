use num::Zero;

pub struct Uint256(num::BigUint);

impl Uint256 {
    pub fn from_hex(n: &str) -> Uint256 {
        Uint256(num::BigUint::parse_bytes(n.as_bytes(), 16).unwrap())
    }

    pub fn from_int(n: i32) -> Uint256 {
        Uint256(num::BigUint::new(vec![n as u32]))
    }

    pub fn to_uint(self) -> u32 {
        self.0.to_str_radix(10).parse::<u32>().unwrap()
    }

    pub fn to_bool(self) -> bool {
        self.0 > num::BigUint::zero()
    }

    pub fn to_hex_str(self) -> String {
        self.0.to_str_radix(16)
    }

    pub fn copy(&self) -> Uint256 {
        Uint256(self.0.clone())
    }
}

impl std::ops::Add for Uint256 {
    type Output = Uint256;

    fn add(self, rhs: Uint256) -> Uint256 {
        Uint256(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Uint256 {
    type Output = Uint256;

    fn sub(self, rhs: Uint256) -> Uint256 {
        Uint256(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Uint256 {
    type Output = Uint256;

    fn mul(self, rhs: Uint256) -> Uint256 {
        Uint256(self.0 * rhs.0)
    }
}

impl std::ops::Div for Uint256 {
    type Output = Uint256;

    fn div(self, rhs: Uint256) -> Uint256 {
        Uint256(self.0 / rhs.0)
    }
}

impl std::fmt::Display for Uint256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

impl std::fmt::Debug for Uint256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}
