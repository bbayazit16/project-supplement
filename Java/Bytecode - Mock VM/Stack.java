import java.math.BigInteger;
import java.util.ArrayList;

public class Stack {
    private ArrayList<BigInteger> stack;

    public Stack() {
        stack = new ArrayList<BigInteger>();
    }

    public void push(BigInteger value) {
        stack.add(value);
    }

    public void push(int value) {
        stack.add(new BigInteger(Integer.toHexString(value), 16));
    }

    public BigInteger pop() {
        return stack.remove(stack.size() - 1);
    }

    public void swap(int a, int b) {
        stack.set(stack.size() - 1 - a, stack.set(stack.size() - 1 - b, stack.get(stack.size() - 1 - a)));
    }

    public int size() {
        return stack.size();
    }

    public String toString() {
        return stack.toString();
        // StringBuilder bdr = new StringBuilder();
        // for (int i = 0; i < stack.size(); i++) {
        //     if (i != stack.size() - 1) {
        //         bdr.append(stack.get(i).toString(16) + "\n");
        //     } else {
        //         bdr.append(stack.get(i).toString(16));
        //     }
        // }
        // return bdr.toString();
    }
}
