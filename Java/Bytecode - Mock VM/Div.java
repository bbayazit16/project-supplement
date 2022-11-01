import java.math.BigInteger;

public class Div implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        stack.push(stack.pop().divide(stack.pop()));
        return new Result(true, null);
    }

    public int getParamCount() {
        return 2;
    }

    public int getIdentifier() {
        return 4;
    }

    public String toString() {
        return "DIV";
    }
}
