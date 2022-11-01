import java.math.BigInteger;

public class Push implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        stack.push(args[0]);
        return new Result(true, null);
    }

    public int getParamCount() {
        return 0;
    }

    public int getIdentifier() {
        return 5;
    }

    public String toString() {
        return "PUSH";
    }
}
