import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;

        HashMap<String, HashMap<String, Integer>> map = new HashMap<String, HashMap<String, Integer>>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                String[] parts = data.split(" = ");
                Integer dist = Integer.parseInt(parts[1]);
                String[] locs = parts[0].split(" to ");
                if (map.containsKey(locs[0])) {
                    HashMap<String, Integer> submap = map.get(locs[0]);
                    submap.put(locs[1], dist);
                    map.put(locs[0], submap);
                } else {
                    HashMap<String, Integer> submap = new HashMap<String, Integer>();
                    submap.put(locs[1], dist);
                    map.put(locs[0], submap);
                }
                if (map.containsKey(locs[1])) {
                    HashMap<String, Integer> submap = map.get(locs[1]);
                    submap.put(locs[0], dist);
                    map.put(locs[1], submap);
                } else {
                    HashMap<String, Integer> submap = new HashMap<String, Integer>();
                    submap.put(locs[0], dist);
                    map.put(locs[1], submap);
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        Integer min = Integer.MAX_VALUE;
        Integer max = 0;
        ArrayList<String> locations = new ArrayList<String>(map.keySet());
        ArrayList<ArrayList<String>> permutations = getPermutations(locations);
        for (ArrayList<String> permutation : permutations) {
            int dist = 0;
            for (int i = 0; i < permutation.size() - 1; i++) {
                dist = dist + map.get(permutation.get(i)).get(permutation.get(i+1));
            }
            if (dist < min) {
                min = dist;
            }
            if (dist > max) {
                max = dist;
            }
        }
        System.out.println("Part 1: " + min);
        System.out.println("Part 2: " + max);
    }

    private static ArrayList<ArrayList<String>> getPermutations(ArrayList<String> locations) {
        int n = locations.size();
        int[] state = new int[n];
        ArrayList<ArrayList<String>> output = new ArrayList<ArrayList<String>>();
        output.add(locations);
        for (int i = 0; i < n; i++) {
            state[i]  = 0;
        }
        int i = 1;
        while (i < n) {
            if (state[i] < i) {
                if (i % 2 == 0) {
                    //swap(locations[0], locations[i])
                    String tmp = locations.get(0);
                    locations.set(0, locations.get(i));
                    locations.set(i, tmp);
                } else {
                    //swap(locations[state[i]], locations[i])
                    String tmp = locations.get(state[i]);
                    locations.set(state[i], locations.get(i));
                    locations.set(i, tmp);
                }
                output.add((ArrayList)locations.clone());
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