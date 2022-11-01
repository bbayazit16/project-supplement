import java.math.BigInteger;

public class Mload implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        stack.push(memory.load(stack.pop().intValue()));
        return new Result(true, null);
    }

    public int getParamCount() {
        return 1;
    }

    public int getIdentifier() {
        return 6;
    }

    public String toString() {
        return "PUSH";
    }
}
