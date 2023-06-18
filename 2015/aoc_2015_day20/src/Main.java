import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        final int input = 29000000;
        part1(input);
        part2(input);
    }

    private static void part1(int input) {
        int house = 1;
        while (true) {
            ArrayList<Integer> factors = factorize(house);
            int res = 0;
            for (Integer i : factors) {
                res = res + (10*i);
            }
            if (res >= input) {
                System.out.println("Part one: " + house);
                break;
            }
            house++;
        }
    }

    private static void part2(int input) {
        int house = 1;
        while (true) {
            ArrayList<Integer> factors = factorizeLimited(house);
            int res = 0;
            for (Integer i : factors) {
                res = res + (11 * i);
            }
            if (res >= input) {
                System.out.println("Part two: " + house);
                break;
            }
            house++;
        }
    }
    private static ArrayList<Integer> factorize(int i) {
        ArrayList<Integer> factors = new ArrayList<>();
        double upperbound = Math.floor(Math.sqrt((double)i));
        for (int j = 1; j <= upperbound; j++) {
            if (i % j == 0) {
                factors.add(j);
                factors.add(i / j);
            }
        }
        return factors;
    }

    private static ArrayList<Integer> factorizeLimited(int i) {
        ArrayList<Integer> factors = new ArrayList<>();
        double upperbound = 50;
        for (int j = 1; j <= upperbound; j++) {
            if (i % j == 0) {
                factors.add(i / j);
            }
        }
        return factors;
    }
}