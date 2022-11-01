mod utils;
mod vm;

use vm::vm::VM;

fn get_time() -> std::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
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
fn main() {
    /*
     * 000131ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff410001400000340020000054
     * Subtract 1 from uint256 max
     * add 1 to subtracted value
     * return value (returning uint256 max)
     */

    /*
     * JMP test, return 1
     * 000750555555555200010000340020000054
     */

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let vm = VM::new(&input);

        let begin = get_time();
        match vm.interpret() {
            Some(ret) => {
                let end = get_time();
                println!("Returned {} in {:#?}", ret, end - begin);
            }
            None => {
                let end = get_time();
                println!("Terminated in {:#?}", end - begin);
            }
        }
    }
}
