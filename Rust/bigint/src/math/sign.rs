use std::{fmt, cmp::Ordering};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

impl std::ops::Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
            Sign::Positive => Sign::Negative,
        }
    }
}

impl std::ops::Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cmp(&rhs) == Ordering::Equal {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if let Sign::Negative = self { "-" } else { "" })
    }
}

impl fmt::Debug for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl Default for Sign {
    fn default() -> Self {
        Self::Zero
    }
}
