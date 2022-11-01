import java.math.BigInteger;

public class Sub implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        stack.push(stack.pop().subtract(stack.pop()));
        return new Result(true, null);
    }

    public int getParamCount() {
        return 2;
    }

    public int getIdentifier() {
        return 2;
    }

    public String toString() {
        return "SUB";
    }
}
