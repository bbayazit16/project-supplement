import java.math.BigInteger;

public class Result {
    private boolean cont;
    private BigInteger ret;

    public Result(boolean shouldContinue, BigInteger returnValue) {
        cont = shouldContinue;
        ret = returnValue;
    }

    public boolean shouldContinue() {
        return cont;
    }

    public BigInteger returnValue() {
        return ret;
    }
}
