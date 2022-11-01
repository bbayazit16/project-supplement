import java.math.BigInteger;

public class Add implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        stack.push(stack.pop().add(stack.pop()));
        return new Result(true, null);
    }

    public int getParamCount() {
        return 2;
    }

    public int getIdentifier() {
        return 1;
    }

    public String toString() {
        return "ADD";
    }
}
