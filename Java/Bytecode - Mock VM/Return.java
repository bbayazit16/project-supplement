import java.math.BigInteger;

public class Return implements Opcode {

    @Override
    public Result execute(Stack stack, Memory memory, BigInteger[] args) {
        String ret = memory.load(stack.pop().intValue()).toString(16);

        String zeropad = "";
        for (int i = 0; i < 32 - ret.length(); i++) {
            zeropad += "0";
        }

        ret = zeropad + ret;

        return new Result(false, new BigInteger(
                ret.substring(0, stack.pop().intValue()), 16));
        // return new Result(false,
        // new BigInteger(
        // memory.load(stack.pop().intValue())
        // .toString(16)
        // .substring(0, stack.pop().intValue()),
        // 16));
    }

    public int getParamCount() {
        return 2;
    }

    public int getIdentifier() {
        return 8;
    }

    public String toString() {
        return "RETURN";
    }
}
