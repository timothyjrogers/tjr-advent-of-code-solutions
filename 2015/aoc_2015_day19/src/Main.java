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
        HashMap<String, ArrayList<String>> rules = new HashMap<String, ArrayList<String>>();
        HashMap<String, String> reversedRules = new HashMap<String, String>();
        String start = "";
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                if (data.isEmpty()) {
                    data = reader.readLine();
                    start = data.trim();
                    break;
                }
                String[] rule = data.split(" => ");
                if (rules.containsKey(rule[0])) {
                    rules.get(rule[0]).add(rule[1]);
                } else {
                    ArrayList<String> r = new ArrayList<String>();
                    r.add(rule[1]);
                    rules.put(rule[0], r);
                }
                reversedRules.put(rule[1], rule[0]);
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        HashSet<String> molecules = new HashSet<String>();
        for (Map.Entry<String, ArrayList<String>> rule : rules.entrySet()) {
            String k = rule.getKey();
            ArrayList<String> v = rule.getValue();
            int index = 0;
            while(index != -1) {
                index = start.indexOf(k, index);
                if (index != -1) {
                    for (String r : v) {
                        StringBuilder builder = new StringBuilder();
                        builder.append(start.substring(0, index));
                        builder.append(r);
                        if (index + k.length() < start.length()) {
                            builder.append(start.substring(index + k.length()));
                        }
                        molecules.add(builder.toString());
                    }
                    index++;
                }
            }
        }
        System.out.println("Part one: " + molecules.size());

        ArrayList<String> sortedKeys = new ArrayList<String>(reversedRules.keySet());
        int keyIndex = 0;
        int steps = 0;
        String orig = start;
        while (!start.equals("e")) {
            if (keyIndex == sortedKeys.size()) {
                Collections.shuffle(sortedKeys);
                keyIndex = 0;
                start = orig;
                steps = 0;
            }
            String k = sortedKeys.get(keyIndex);
            String v = reversedRules.get(k);
            if (!start.contains(k)) {
                keyIndex++;
                continue;
            } else {
                int index = 0;
                while(index != -1) {
                    index = start.indexOf(k, index);
                    if (index != -1) {
                        StringBuilder builder = new StringBuilder();
                        builder.append(start.substring(0, index));
                        builder.append(v);
                        if (index + k.length() < start.length()) {
                            builder.append(start.substring(index + k.length()));
                        }
                        start = builder.toString();
                        index++;
                        steps++;
                    }
                }
                keyIndex = 0;
            }
        }
        System.out.println("Part two: " + steps);
    }
}