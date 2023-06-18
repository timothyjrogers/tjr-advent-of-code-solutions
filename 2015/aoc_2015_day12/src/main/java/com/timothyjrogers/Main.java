package com.timothyjrogers;
import java.io.*;
import java.util.Iterator;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.JsonNode;
public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        final InputStream stream = Main.class.getClassLoader().getResourceAsStream(inputFile);
        final InputStreamReader streamReader = new InputStreamReader(stream);
        final BufferedReader reader = new BufferedReader(streamReader);
        String data = "";
        try {
            data = reader.readLine();
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        part1(data);
        part2(data);
    }

    private static void part1(String input) {
        int sum = 0;
        char[] chars = input.toCharArray();
        StringBuilder builder = new StringBuilder();
        int mod = 1;
        for (int i = 0; i < chars.length; i++) {
            if (Character.isDigit(chars[i])) {
                if (builder.isEmpty()) {
                    if (chars[i - 1] == '-') {
                        mod = -1;
                    } else {
                        mod = 1;
                    }
                }
                builder.append(chars[i]);
            } else if (!builder.isEmpty()) {
                sum = sum + mod * Integer.parseInt(builder.toString());
                builder.setLength(0);
            }
        }
        System.out.println(sum);
    }

    private static void part2(String input) {
        ObjectMapper mapper = new ObjectMapper();
        try {
            JsonNode root = mapper.readTree(input);
            System.out.println(sumNodes(root));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static int sumNodes(JsonNode node) {
        int sum = 0;
        if (node.isInt()) {
            return node.asInt();
        }
        if (node.isArray()) {
            for (JsonNode n : node) {
                sum += sumNodes(n);
            }
            return sum;
        }
        if (node.isObject() && !containsRed(node)) {
            Iterator<JsonNode> elements = node.elements();
            while (elements.hasNext()) {
                JsonNode cur = elements.next();
                sum += sumNodes(cur);
            }
            return sum;
        }
        return sum;
    }

    private static boolean containsRed(JsonNode node) {
        Iterator<JsonNode> iter = node.iterator();
        boolean res = false;
        while (iter.hasNext()) {
            JsonNode cur = iter.next();
            if (cur.asText().equals("red")) {
                res = true;
                break;
            }
        }
        return res;
    }
}