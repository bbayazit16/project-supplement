mod sign;
use sign::Sign;

type Bytes = Vec<u8>;

mod integer;

pub type Z = integer::Integer;

#[macro_export]
macro_rules! int {
    ($i: expr) => {{
        let z: Z = stringify!($i).try_into().unwrap();
        z
    }};
}

pub use int;
