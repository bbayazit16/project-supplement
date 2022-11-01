import java.math.BigInteger;

interface Opcode {
    public int getIdentifier();

    public int getParamCount();

    public Result execute(Stack stack, Memory memory, BigInteger[] args);
}