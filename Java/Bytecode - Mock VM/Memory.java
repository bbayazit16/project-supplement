import java.math.BigInteger;
import java.util.ArrayList;

public class Memory {
    private ArrayList<Character> memory;

    public Memory() {
        memory = new ArrayList<Character>();
    }

    public void store(int index, BigInteger value) {
        String hex = value.toString(16);

        if (index >= memory.size()) {
            for (int i = 0; i < 32; i++) {
                memory.add('0');
            }
        }

        assert hex.length() > 32;

        String zeropad = "";
        for (int i = 0; i < 32 - hex.length(); i++) {
            zeropad += "0";
        }

        hex = zeropad + hex;

        for (int i = 0; i < hex.length(); i++) {
            memory.set(index + i, hex.charAt(i));
        }
    }

    public BigInteger load(int index) {
        String hexcode = "";
        for (int i = 0; i < 32; i++) {
            hexcode += String.valueOf(memory.get(i));
        }
        return new BigInteger(hexcode, 16);
    }

    public String toString() {
        if (memory.size() == 0) {
            return "[]";
        }

        StringBuilder bdr = new StringBuilder(memory.size());
        for (int i = 0; i < memory.size(); i++) {
            if (i % 32 == 0) {
                int mult = i / 32;
                if (mult != 0) {
                    bdr.append('\n');
                }
                bdr.append("[" + mult + "]: ");
            }
            bdr.append(memory.get(i));
        }
        return bdr.toString();
    }
}
