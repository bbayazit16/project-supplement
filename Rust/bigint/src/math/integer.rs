use std::{cmp::Ordering, fmt};

use crate::math::{Bytes, Sign};

#[derive(Eq, PartialEq, Clone)]
pub struct Integer {
    sign: Sign,
    bytes: Bytes,
}

impl Integer {
    pub fn zero() -> Self {
        Self {
            sign: Sign::Zero,
            bytes: vec![0],
        }
    }

    pub fn one() -> Self {
        Self {
            sign: Sign::Positive,
            bytes: vec![1],
        }
    }

    pub fn ten() -> Self {
        Self {
            sign: Sign::Positive,
            bytes: vec![1, 0],
        }
    }

    pub fn ten_pow(exp: u32) -> Self {
        let mut bytes = vec![1];
        bytes.append(&mut vec![0; exp as usize]);

        Self {
            sign: Sign::Positive,
            bytes,
        }
    }

    pub fn ten_mul(&self, exp: u32) -> Self {
        let mut bytes = self.bytes.clone();
        bytes.append(&mut vec![0; exp as usize]);

        Self {
            sign: self.sign,
            bytes,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.sign == Sign::Positive
    }

    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    pub fn is_zero(&self) -> bool {
        self.sign == Sign::Zero
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn be_bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    pub fn le_bytes(&self) -> Bytes {
        let mut be = self.bytes.clone();
        be.reverse();
        be
    }

    pub fn arithemtic_shr(&mut self, c: usize) -> Bytes {
        let mut bytes = vec![];
        for _ in 0..c {
            bytes.insert(0, self.bytes.pop().unwrap_or(0));
        }

        bytes
    }

    fn clean_bytes(bytes: &mut Bytes) {
        for _ in 0..bytes.len() {
            if bytes[0] != 0 {
                break;
            }
            bytes.remove(0);
        }

        if bytes.is_empty() {
            bytes.push(0);
        }
    }

    fn compare_bytes(a: &Bytes, b: &Bytes) -> Ordering {
        if a.len() < b.len() {
            Ordering::Less
        } else if a.len() > b.len() {
            Ordering::Greater
        } else {
            for (c, d) in a.iter().zip(b.iter()) {
                if c > d {
                    return Ordering::Greater;
                }

                if c < d {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }

    pub fn dual_mul(v: Self, w: Self) -> Self {
        if v.bytes == vec![0] || w.bytes == vec![0] {
            return Self::zero();
        }

        let mut missing_digit_count = 0;

        let a = v.bytes.get(0).unwrap_or_else(|| {
            missing_digit_count += 1;
            &0
        });

        let b = v.bytes.get(1).unwrap_or_else(|| {
            missing_digit_count += 1;
            &0
        });

        let c = w.bytes.get(0).unwrap_or_else(|| {
            missing_digit_count += 1;
            &0
        });

        let d = w.bytes.get(1).unwrap_or_else(|| {
            missing_digit_count += 1;
            &0
        });

        let e = d * b;

        let f = d * a + (e / 10);
        let e = e % 10;

        let mut bytes = vec![f / 10, f % 10, e];
        Self::clean_bytes(&mut bytes);

        let x = Self {
            sign: Sign::Positive,
            bytes,
        };

        let e = c * b;
        let f = c * a + (e / 10);
        let e = e % 10;

        let mut bytes = vec![f / 10, f % 10, e, 0];
        Self::clean_bytes(&mut bytes);

        let y = Self {
            sign: Sign::Positive,
            bytes,
        };

        let mut ret = x + y;

        if ret.bytes.len() != 1 {
            ret.arithemtic_shr(missing_digit_count);
        }

        ret
    }
}

impl std::cmp::PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        let sign_eq = self.sign.cmp(&other.sign);
        if sign_eq != Ordering::Equal {
            return sign_eq;
        }

        match self.sign {
            Sign::Positive => Self::compare_bytes(&self.bytes, &other.bytes),
            Sign::Negative => Self::compare_bytes(&other.bytes, &self.bytes),
            Sign::Zero => Ordering::Equal,
        }
    }
}

impl std::ops::Neg for Integer {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.sign = -self.sign;
        self
    }
}

impl std::ops::Add for Integer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Assign LHS the longest, RHS the shortest integer
        let (lhs, rhs) = if self.len() >= rhs.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        match lhs.sign.cmp(&rhs.sign) {
            Ordering::Less => rhs - (-lhs),

            Ordering::Greater => lhs - (-rhs),
            Ordering::Equal => {
                let mut sign = lhs.sign;
                let mut bytes = vec![];
                let mut carry = 0;

                for (a, b) in lhs
                    .bytes
                    .iter()
                    .rev()
                    .zip(rhs.bytes.iter().rev().chain(std::iter::repeat(&0)))
                {
                    let sum = a + b + carry;
                    carry = if sum >= 10 { 1 } else { 0 };
                    bytes.insert(0, sum % 10);
                }

                if carry != 0 {
                    bytes.insert(0, carry);
                }

                if bytes.is_empty() {
                    bytes.push(0);
                    sign = Sign::Zero;
                }

                Self { bytes, sign }
            }
        }
    }
}

impl std::ops::Sub for Integer {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs == Self::zero() {
            return self;
        }

        match self.sign.cmp(&rhs.sign) {
            Ordering::Less => -(-self + rhs),
            Ordering::Greater => self + (-rhs),
            Ordering::Equal => {
                if self.len() == rhs.len() {
                    if self.sign == Sign::Positive && self < rhs {
                        return -(rhs - self);
                    } else if self.sign == Sign::Negative && self > rhs {
                        return -rhs - -self;
                    }
                }

                let mut sign = {
                    if self > rhs {
                        if self.sign != Sign::Positive {
                            -(self.sign)
                        } else {
                            self.sign
                        }
                    } else {
                        if self.sign == Sign::Positive {
                            -self.sign
                        } else {
                            self.sign
                        }
                    }
                };

                // Assign LHS the longest, RHS the shortest integer
                let (lhs, rhs) = if self.len() >= rhs.len() {
                    (self, rhs)
                } else {
                    (rhs, self)
                };

                let mut bytes = vec![];
                let mut carry = 0;
                for (a, b) in lhs
                    .bytes
                    .iter()
                    .rev()
                    .zip(rhs.bytes.iter().rev().chain(std::iter::repeat(&0)))
                {
                    if a >= b {
                        if *a < b + carry {
                            let diff = 10 + a - carry - b;
                            carry = 1;
                            bytes.insert(0, diff);
                        } else {
                            let diff = a - carry - b;
                            carry = 0;
                            bytes.insert(0, diff);
                        }
                    } else {
                        let diff = 10 + a - carry - b;
                        carry = 1;
                        bytes.insert(0, diff);
                    }
                }

                Self::clean_bytes(&mut bytes);

                if bytes.is_empty() {
                    bytes.push(0);
                    sign = Sign::Zero;
                }

                Self { bytes, sign }
            }
        }
    }
}

impl std::ops::Mul for Integer {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign * rhs.sign;

        if self.len() <= 2 && rhs.len() <= 2 {
            return Self::dual_mul(self, rhs);
        }

        let self_dc = self.len();
        let rhs_dc = rhs.len();

        let mut n = self_dc.max(rhs_dc);

        if n % 2 != 0 {
            n += 1;
        };

        let shift_count = (n - self_dc) + (n - rhs_dc);

        let mut a = self.ten_mul((n - self_dc) as u32);
        let mut c = rhs.ten_mul((n - rhs_dc) as u32);

        let half_n = n / 2;

        let mut b = a.arithemtic_shr(half_n);

        Self::clean_bytes(&mut b);

        let b = Self {
            sign: Sign::Positive,
            bytes: b,
        };

        let mut d = c.arithemtic_shr(half_n);

        Self::clean_bytes(&mut d);

        let d = Self {
            sign: Sign::Positive,
            bytes: d,
        };

        let ac = a.clone() * c.clone();
        let bd = b.clone() * d.clone();
        let ad_bc = (a + b) * (c + d) - ac.clone() - bd.clone();

        let mut ret = ac.ten_mul(n as u32) + ad_bc.ten_mul(half_n as u32) + bd;
        ret.arithemtic_shr(shift_count);
        ret.sign = sign;

        ret
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bdr = self.sign.to_string();

        bdr.extend(self.bytes.iter().map(u8::to_string));

        write!(f, "{}", bdr)
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl From<i32> for Integer {
    fn from(mut i: i32) -> Self {
        let sign = if i.is_positive() {
            Sign::Positive
        } else if i.is_negative() {
            Sign::Negative
        } else {
            Sign::Zero
        };

        let mut bytes = vec![];

        while i != 0 {
            bytes.insert(0, (i % 10) as u8);
            i /= 10;
        }

        Self { sign, bytes }
    }
}

impl TryFrom<&str> for Integer {
    type Error = Box<dyn std::error::Error>;

    fn try_from(num_string: &str) -> Result<Self, Self::Error> {
        if num_string.is_empty() {
            return Ok(Self::zero());
        }

        let mut iter = num_string.chars();
        let mut bytes = vec![];

        let first = iter.next().unwrap();
        let sign = match first {
            '+' => Sign::Positive,
            '-' => Sign::Negative,
            '0' => {
                if num_string.len() != 1 {
                    Err("Invalid integer string")?
                }

                return Ok(Self::zero());
            }
            '1'..='9' => {
                bytes.push(first.to_digit(10).unwrap() as u8);
                Sign::Positive
            }
            _ => return Err("Invalid integer string")?,
        };

        for chr in iter {
            let digit = chr.to_digit(10).ok_or("Invalid integer string")?;
            bytes.push(digit as u8)
        }

        if sign != Sign::Zero && bytes[0] == 0 {
            return Err("Invalid integer string")?;
        }

        Ok(Self { bytes, sign })
    }
}
