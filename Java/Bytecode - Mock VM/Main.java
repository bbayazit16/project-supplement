import java.math.BigInteger;
import java.util.HashMap;

public class Main {

    public static void main(String[] args) {

        final String bytecode = "0520052003050007 05 20 05 00 08".replaceAll(" ", "");

        Stack stack = new Stack();
        Memory memory = new Memory();

        HashMap<String, Opcode> map = new HashMap<String, Opcode>();

        for (Opcode code : new Opcode[] {
                new Add(), new Sub(), new Mul(), new Div(),
                new Push(), new Mload(), new Mstore(), new Return()
        }) {
            String id = Integer.toHexString(code.getIdentifier());
            if (id.length() == 1) {
                id = "0" + id;
            }
            map.put(id, code);
        }

        for (int i = 0; i < bytecode.length() - 1; i += 2) {
            String code = bytecode.substring(i, i + 2);
            Opcode operation = map.get(code);

            Result res;
            if (operation == null) {
                if (map.get(bytecode.substring(i - 2, i)).toString().equals("PUSH")) {
                    continue;
                }
            }
            if (operation.toString().equals("PUSH")) {
                res = operation.execute(stack, memory, new BigInteger[] {
                        new BigInteger(bytecode.substring(i + 2, i + 4), 16)
                });
            } else {
                res = operation.execute(stack, memory, new BigInteger[] {});
            }

            if (!res.shouldContinue()) {
                System.out.println("Terminating...");
                if (res.returnValue() != null) {
                    System.out.println(stack);
                    System.out.println(memory);
                    System.out.println("Return value: " + res.returnValue().toString(16));
                }
                return;
            }
        }

        System.out.println(stack);
        System.out.println(memory);
        System.out.println("Operation executed successfully.");

    }
}