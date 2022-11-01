import java.math.BigInteger;

public class Mstore implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        memory.store(stack.pop().intValue(), stack.pop());
        return new Result(true, null);
    }

    public int getParamCount() {
        return 2;
    }

    public int getIdentifier() {
        return 7;
    }

    public String toString() {
        return "MSTORE";
    }
}
