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
        HashMap<String, HashMap<String, Integer>> adjacency = new HashMap<String, HashMap<String, Integer>>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                String[] parts = data.split(" ");
                String name1 = parts[0];
                String name2 = parts[10].substring(0, parts[10].length() - 1);
                String modifier = parts[2];
                Integer value = Integer.parseInt(parts[3]);
                if (modifier.equals("lose")) {
                    value = value * -1;
                }
                if (adjacency.containsKey(name1)) {
                    HashMap<String,Integer> relation = adjacency.get(name1);
                    relation.put(name2, value);
                    adjacency.put(name1, relation);
                } else {
                    HashMap<String, Integer> relation = new HashMap<String, Integer>();
                    relation.put(name2, value);
                    adjacency.put(name1, relation);
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        part1(adjacency);
        part2(adjacency);
    }

    private static void part1(HashMap<String, HashMap<String, Integer>> adjacency) {
        ArrayList<String> names = new ArrayList<String>(adjacency.keySet());
        ArrayList<ArrayList<String>> permutations = getPermutations(names);
        Integer max = 0;
        for (ArrayList<String> permutation : permutations) {
            Integer happiness = 0;
            for (int i = 0; i < permutation.size()-1; i++) {
                happiness += adjacency.get(permutation.get(i)).get(permutation.get(i+1));
                happiness += adjacency.get(permutation.get(i+1)).get(permutation.get(i));
            }
            happiness += adjacency.get(permutation.get(permutation.size()-1)).get(permutation.get(0));
            happiness += adjacency.get(permutation.get(0)).get(permutation.get(permutation.size()-1));
            if (happiness > max) {
                max = happiness;
            }
        }
        System.out.println(max);
    }
    private static void part2(HashMap<String, HashMap<String, Integer>> adjacency) {
        HashMap<String, HashMap<String, Integer>> adjacencyWithMe = adjacency;
        HashMap<String, Integer> myRelation = new HashMap<String, Integer>();
        ArrayList<String> names = new ArrayList<String>(adjacency.keySet());
        for (String name : names) {
            HashMap<String, Integer> relation = adjacencyWithMe.get(name);
            relation.put("Me", 0);
            adjacencyWithMe.put(name, relation);
            myRelation.put(name, 0);
        }
        adjacencyWithMe.put("Me", myRelation);
        names.add("Me");
        ArrayList<ArrayList<String>> permutations = getPermutations(names);
        Integer max = 0;
        for (ArrayList<String> permutation : permutations) {
            Integer happiness = 0;
            for (int i = 0; i < permutation.size()-1; i++) {
                happiness += adjacency.get(permutation.get(i)).get(permutation.get(i+1));
                happiness += adjacency.get(permutation.get(i+1)).get(permutation.get(i));
            }
            happiness += adjacency.get(permutation.get(permutation.size()-1)).get(permutation.get(0));
            happiness += adjacency.get(permutation.get(0)).get(permutation.get(permutation.size()-1));
            if (happiness > max) {
                max = happiness;
            }
        }
        System.out.println(max);
    }
    private static ArrayList<ArrayList<String>> getPermutations(ArrayList<String> items) {
        int n = items.size();
        int[] state = new int[n];
        ArrayList<ArrayList<String>> output = new ArrayList<ArrayList<String>>();
        output.add(items);
        for (int i = 0; i < n; i++) {
            state[i]  = 0;
        }
        int i = 1;
        while (i < n) {
            if (state[i] < i) {
                if (i % 2 == 0) {
                    //swap(locations[0], locations[i])
                    String tmp = items.get(0);
                    items.set(0, items.get(i));
                    items.set(i, tmp);
                } else {
                    //swap(locations[state[i]], locations[i])
                    String tmp = items.get(state[i]);
                    items.set(state[i], items.get(i));
                    items.set(i, tmp);
                }
                output.add((ArrayList)items.clone());
                state[i] += 1;
                i = 1;
            } else {
                state[i] = 0;
                i++;
            }
        }
        return output;
    }
}