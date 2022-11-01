import java.math.BigInteger;

public class Mul implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        stack.push(stack.pop().multiply(stack.pop()));
        return new Result(true, null);
    }

    public int getParamCount() {
        return 2;
    }

    public int getIdentifier() {
        return 3;
    }

    public String toString() {
        return "MUL";
    }
}
