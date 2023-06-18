import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";

        String data = "";
        BufferedReader reader;

        ArrayList<String> instructions = new ArrayList<String>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                instructions.add(data);
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        Integer p1res = part1((ArrayList)instructions.clone());
        part2(instructions, p1res);
    }

    private static Integer part1(ArrayList<String> instructions) {
        HashMap<String, Integer> wires = new HashMap<String, Integer>();
        while (!instructions.isEmpty()) {
            String instruction = instructions.remove(0);
            String[] parts = instruction.split(" -> ");
            String wire = parts[1];
            String[] signal = parts[0].split(" ");
            if (signal.length == 1) { //static value
                try {
                    wires.put(wire, Integer.parseInt(parts[0]) & 0xFFFF);
                } catch (NumberFormatException e) {
                    if (!wires.containsKey(parts[0])) {
                        instructions.add(instruction);
                        continue;
                    }
                    wires.put(wire, wires.get(parts[0]));
                }
            } else if (signal.length == 2) {
                if (!wires.containsKey(signal[1])) {
                    instructions.add(instruction);
                    continue;
                }
                Integer val = (~wires.get(signal[1])) & 0xFFFF;
                wires.put(wire, val);
            } else if (signal.length == 3) {
                Integer operand1 = 0;
                Integer operand2 = 0;
                Integer val = 0;
                try {
                    operand1 = Integer.parseInt(signal[0]);
                } catch (NumberFormatException e) {
                    if (!wires.containsKey(signal[0])) {
                        instructions.add(instruction);
                        continue;
                    }
                    operand1 = wires.get(signal[0]);
                }
                try {
                    operand2 = Integer.parseInt(signal[2]);
                } catch (NumberFormatException e) {
                    if (!wires.containsKey(signal[2])) {
                        instructions.add(instruction);
                        continue;
                    }
                    operand2 = wires.get(signal[2]);
                }
                if (signal[1].equals("AND")) {
                    val = (operand1 & operand2) & 0xFFFF;
                } else if (signal[1].equals("OR")) {
                    val = (operand1 | operand2) & 0xFFFF;
                } else if (signal[1].equals("LSHIFT")) {
                    val = (operand1 << operand2) & 0xFFFF;
                } else if (signal[1].equals("RSHIFT")) {
                    val = (operand1 >> operand2) & 0xFFFF;
                }
                wires.put(wire, val);
            }
        }
        System.out.println(wires.get("a"));
        return wires.get("a");
    }

    private static void part2(ArrayList<String> instructions, Integer p1) {
        HashMap<String, Integer> wires = new HashMap<String, Integer>();
        while (!instructions.isEmpty()) {
            String instruction = instructions.remove(0);
            if (instruction.endsWith("-> b")) {
                instruction = p1 + " -> b";
            }
            String[] parts = instruction.split(" -> ");
            String wire = parts[1];
            String[] signal = parts[0].split(" ");
            if (signal.length == 1) { //static value
                try {
                    wires.put(wire, Integer.parseInt(parts[0]) & 0xFFFF);
                } catch (NumberFormatException e) {
                    if (!wires.containsKey(parts[0])) {
                        instructions.add(instruction);
                        continue;
                    }
                    wires.put(wire, wires.get(parts[0]));
                }
            } else if (signal.length == 2) {
                if (!wires.containsKey(signal[1])) {
                    instructions.add(instruction);
                    continue;
                }
                Integer val = (~wires.get(signal[1])) & 0xFFFF;
                wires.put(wire, val);
            } else if (signal.length == 3) {
                Integer operand1 = 0;
                Integer operand2 = 0;
                Integer val = 0;
                try {
                    operand1 = Integer.parseInt(signal[0]);
                } catch (NumberFormatException e) {
                    if (!wires.containsKey(signal[0])) {
                        instructions.add(instruction);
                        continue;
                    }
                    operand1 = wires.get(signal[0]);
                }
                try {
                    operand2 = Integer.parseInt(signal[2]);
                } catch (NumberFormatException e) {
                    if (!wires.containsKey(signal[2])) {
                        instructions.add(instruction);
                        continue;
                    }
                    operand2 = wires.get(signal[2]);
                }
                if (signal[1].equals("AND")) {
                    val = (operand1 & operand2) & 0xFFFF;
                } else if (signal[1].equals("OR")) {
                    val = (operand1 | operand2) & 0xFFFF;
                } else if (signal[1].equals("LSHIFT")) {
                    val = (operand1 << operand2) & 0xFFFF;
                } else if (signal[1].equals("RSHIFT")) {
                    val = (operand1 >> operand2) & 0xFFFF;
                }
                wires.put(wire, val);
            }
        }
        System.out.println(wires.get("a"));
    }
}