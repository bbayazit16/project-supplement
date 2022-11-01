use crate::utils::bytecode::Bytecode;
use crate::utils::uint256::Uint256;
use crate::vm::memory::Memory;
use crate::vm::stack::Stack;

pub struct VM {
    stack: Stack,
    memory: Memory,
    bytecode: Bytecode,
}

/**
 *
 * Storage Management:
 *
 * 0 => PUSH1
 * 1 => PUSH2
 * ...
 * 31 => PUSH32
 *
 * 32 => POP
 * 33 => SWAP
 * 34 => MSTORE
 * 35 => MLOAD
 * 36 => DUP
 * 37 => SWAP
 *
 * Operations:
 * 40 => ADD
 * 41 => SUB
 * 42 => MUL
 * 43 => DIV
 *
 * Control Flow:
 * 50 => JUMP
 * 51 => JUMPI
 * 52 => JUMPDEST
 * 53 => PC
 * 54 => RETURN
 * 55 => HALT
 */
impl VM {
    pub fn new(bytecode: &str) -> Self {
        VM {
            stack: Stack::new(),
            memory: Memory::new(),
            bytecode: Bytecode::from_hex_str(bytecode),
        }
    }

    pub fn interpret(mut self) -> Option<String> {
        let mut n = false;

        let mut i: usize = 0;
        while i < self.bytecode.len() {
            let opcode = self.bytecode.get(i).copy();

            if n {
                n = !n;
                self.stack.push(opcode);
                i += 1;
                continue;
            }

            let tok = opcode.to_uint();

            if tok < 32 {
                n = !n;
                i += 1;
                continue;
            }

            match tok {
                32 => {
                    self.stack.pop();
                }
                33 => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.swap(a, b);
                }
                34 => {
                    let pos = self.stack.pop();
                    let val = self.stack.pop();
                    self.memory.store(pos, val);
                }
                35 => {
                    let pos = self.stack.pop();
                    self.stack.push(self.memory.load(pos));
                }
                36 => {
                    let a = self.stack.pop();
                    self.stack.push(a.copy());
                    self.stack.push(a);
                }
                37 => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.swap(a, b);
                }
                40 => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a + b);
                }
                41 => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a - b);
                }
                42 => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a * b);
                }
                43 => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a / b);
                }
                50 => {
                    i = self.stack.pop().to_uint() as usize + 1;
                    if self.bytecode.get(i - 1).copy().to_uint() != 52 {
                        println!("Invalid JMP at {}", i);
                        return None;
                    }
                    continue;
                }
                51 => {
                    let dest = self.stack.pop().to_uint() as usize;
                    if self.stack.pop().to_bool() {
                        if self.bytecode.get(dest).copy().to_uint() != 52 {
                            println!("Invalid JMP at {}", i);
                            return None;
                        }
                        i = dest + 1;
                    }
                    continue;
                }
                52 => {
                    i += 1;
                    continue
                },
                53 => {
                    self.stack.push(Uint256::from_int(i as i32));
                }
                54 => {
                    let pos = self.stack.pop();
                    let var = self.stack.pop();
                    return Some(self.memory.load_var(pos, var));
                }
                55 => break,
                _ => return None,
            }

            i += 1;
        }
        println!("Stack: {}", self.stack);
        println!("Memory: {}", self.memory);
        None
    }
}
