import com.sun.jdi.IntegerValue;

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
        ArrayList<Integer> gifts = new ArrayList<>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                gifts.add(Integer.parseInt(data));
                data = reader.readLine();
            };
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        System.out.println("Part one: " + solve(gifts, 3));
        System.out.println("Part two: " + solve(gifts, 4));
    }

    private static long solve(ArrayList<Integer> gifts, int numGroups) {
        int groupWeight = 0;
        for (Integer g : gifts) {
            groupWeight += g;
        }
        groupWeight = groupWeight/numGroups;
        ArrayList<ArrayList<Integer>> groups = new ArrayList<>();
        sum_to_target(gifts, groupWeight, new ArrayList<Integer>(), groups);
        int minGroupSize = Integer.MAX_VALUE;
        for (ArrayList<Integer> g : groups) {
            if (g.size() < minGroupSize) {
                minGroupSize = g.size();
            }
        }
        long minQE = Long.MAX_VALUE;
        for (ArrayList<Integer> g : groups) {
            if (g.size() == minGroupSize) {
                long qe = 1;
                for (Integer x : g) {
                    qe = qe * x;
                }
                if (qe < minQE) {
                    minQE = qe;
                }
            }
        }
        return minQE;
    }
    private static void sum_to_target(ArrayList<Integer> weights, Integer goal, ArrayList<Integer> container, ArrayList<ArrayList<Integer>> groups) {
        int s = 0;
        ArrayList<Integer> result = new ArrayList<>();
        for (int x: container) {
            s += x;
            result.add(x);
        }
        if (s == goal)
            groups.add(result);
        if (s >= goal)
            return;
        for(int i=0; i < weights.size(); i++) {
            ArrayList<Integer> remaining = new ArrayList<Integer>();
            int n = weights.get(i);
            for (int j = i + 1; j < weights.size(); j++) {
                remaining.add(weights.get(j));
            }
            ArrayList<Integer> partial_rec = new ArrayList<Integer>(container);
            partial_rec.add(n);
            sum_to_target(remaining,goal,partial_rec, groups);
        }
    }
}