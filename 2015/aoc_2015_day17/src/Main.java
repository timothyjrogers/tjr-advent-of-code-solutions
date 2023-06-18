import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.String;
import java.util.ArrayList;
import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        ArrayList<Integer> sizes = new ArrayList<Integer>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                sizes.add(Integer.parseInt(data));
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        ArrayList<ArrayList<Integer>> subsets = new ArrayList<ArrayList<Integer>>();
        for (int i = 0; i < (1 << sizes.size()); i++) {
            ArrayList<Integer> subset = new ArrayList<Integer>();
            for (int j = 0; j < sizes.size(); j++) {
                if ((i & (1 << j)) > 0) {
                    subset.add(sizes.get(j));
                }
            }
            subsets.add(subset);
        }
        int count = 0;
        HashMap<Integer, Integer> comboCounts = new HashMap<Integer, Integer>();
        int smallest = Integer.MAX_VALUE;
        for (ArrayList<Integer> subset : subsets) {
            int sum = 0;
            for (Integer i : subset) {
                sum += i;
            }
            if (sum == 150) {
                count++;
                if (comboCounts.containsKey(subset.size())) {
                    comboCounts.put(subset.size(), comboCounts.get(subset.size()) + 1);
                } else {
                    comboCounts.put(subset.size(), 1);
                }
                if (subset.size() < smallest) {
                    smallest = subset.size();
                }
            }
        }
        System.out.println("Part one: " + count);
        System.out.println("Part two: " + comboCounts.get(smallest));
    }
}