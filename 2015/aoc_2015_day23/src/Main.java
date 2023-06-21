import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        ArrayList<String> instructions = new ArrayList<>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                instructions.add(data);
                data = reader.readLine();
            };
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        System.out.println("Part one: " + runProgram(instructions, 0, 0));
        System.out.println("Part two: " + runProgram(instructions, 1, 0));
    }

    private static int runProgram(ArrayList<String> instructions, int a0, int b0) {
        int pc = 0;
        HashMap<String, Integer> registers = new HashMap<>();
        registers.put("a", a0);
        registers.put("b", b0);
        while (pc < instructions.size()) {
            String instruction = instructions.get(pc);
            String[] parts = instruction.split(" ");
            if (parts.length == 3) {
                parts[1] = parts[1].substring(0,1);
            }
            if (!instruction.startsWith("j")) {
                if (parts[0].equals("hlf")) {
                    registers.put(parts[1], registers.get(parts[1]) / 2);
                } else if (parts[0].equals("tpl")) {
                    registers.put(parts[1], registers.get(parts[1]) * 3);
                } else if (parts[0].equals("inc")) {
                    registers.put(parts[1], registers.get(parts[1]) + 1);
                }
                pc++;
            } else if (!instruction.startsWith("ji")) {
                pc += Integer.parseInt(parts[1]);
            } else {
                if (parts[0].equals("jie")) {
                    if (registers.get(parts[1]) % 2 == 0) {
                        pc += Integer.parseInt(parts[2]);
                    } else {
                        pc++;
                    }
                } else if (parts[0].equals("jio")) {
                    if (registers.get(parts[1]) == 1) {
                        pc += Integer.parseInt(parts[2]);
                    } else {
                        pc++;
                    }
                }
            }
        }
        return registers.get("b");
    }
}