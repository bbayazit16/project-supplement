use super::utils::bit_size;

pub struct VM(Vec<u128>);

/*
 * 0 => BUILTIN CALL
 *
 * Operations
 * ----------------
 * 1 => ADD 0x01
 * 2 => MUL 0x02
 * 3 => SUB 0x03
 * 4 => DIV 0x04
 * 5 => MOD 0x05
 * 6 => POW 0x06
 * 7 => LT 0x07
 * 8 => GT 0x08
 * 9 => EQ 0x09
 * 10 => ISZERO 0x0A
 * 11 => AND 0x0B
 * 12 => OR 0x0C
 * 13 => XOR 0x0D
 * 14 => NOT 0x0E
 * 15 => SHL 0x0F
 * 16 => SHR 0x10
 * 17 => MULMOD 0x11
 *
 * ----------------
 *
 * Storage Management
 * ----------------
 * 20 => PUSH1 0x14
 * 21 => PUSH2 0x15
 * ..
 * 51 => PUSH32 0x33
 *
 * 60 => DUP1 0x3c
 * 61 => DUP2 0x3d
 * ..
 * 75 => DUP16 0x4b
 *
 * 80 => SWAP1 0x50
 * 81 => SWAP2 0x51
 * ..
 * 95 => SWAP16 0x5f
 * ----------------
 *
 * 100 => POP 0x64
 * 101 => MLOAD 0x65
 * 102 => MSTORE 0x66
 * 103 => MSIZE 0x67
 *
 * Control Flow
 * ----------------
 * 110 => PC 0x6e
 * 111 => JMPDEST 0x6f
 * 112 => JMP 0x70
 * 113 => JMPI 0x71
 * 114 => RETURN 0x72
 */
impl VM {
    pub fn new(bytecode: &str) -> Result<Self, String> {
        if bytecode.len() % 2 != 0 {
            return Err(String::from("Bytecode must have even length"));
        }
        let mut idx: usize = 0;
        let mut v: Vec<u128> = Vec::new();
        while idx < bytecode.len() - 1 {
            match u128::from_str_radix(&bytecode[idx..idx + 2], 16) {
                Ok(tok) => {
                    v.push(tok);
                    match tok {
                        20..=51 => {
                            let ps = (tok as usize - 19) * 2 + 2;
                            match u128::from_str_radix(&bytecode[idx + 2..ps], 16) {
                                Ok(pv) => {
                                    v.push(pv);
                                    idx += ps;
                                    continue;
                                }
                                Err(_) => {
                                    return Err(format!(
                                        "Push refers to a non-hex value at {}",
                                        idx
                                    ))
                                }
                            }
                        }
                        _ => idx += 2,
                    }
                }
                Err(_) => return Err(format!("Non-hex value at {}", idx)),
            }
        }
        Ok(Self(v))
    }

    pub fn interpret(self) -> Option<u128> {
        let mut idx = 0;
        Some(1)
    }
}
