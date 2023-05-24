import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashSet;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        part1(data);
        part2(data);
    }

    private static void part1(String data) {
        HashSet<ArrayList<Integer>> houses = new HashSet<ArrayList<Integer>>();
        int x = 0;
        int y = 0;
        for(char c : data.toCharArray()) {
            ArrayList<Integer> house = new ArrayList<Integer>();
            house.add(x);
            house.add(y);
            houses.add(house);
            if (c == '^') {
                y = y + 1;
            } else if (c == '>') {
                x = x + 1;
            } else if (c == 'v') {
                y = y - 1;
            } else if (c == '<') {
                x = x - 1;
            }
        }
        ArrayList<Integer> house = new ArrayList<Integer>();
        house.add(x);
        house.add(y);
        houses.add(house);
        System.out.println(houses.size());
    }

    private static void part2(String data) {
        HashSet<ArrayList<Integer>> houses = new HashSet<ArrayList<Integer>>();
        int x1 = 0;
        int y1 = 0;
        int x2 = 0;
        int y2 = 0;
        boolean robot = false;
        for(char c : data.toCharArray()) {
            ArrayList<Integer> house = new ArrayList<Integer>();
            if (robot) {
                house.add(x1);
                house.add(y1);
                houses.add(house);
                if (c == '^') {
                    y1 = y1 + 1;
                } else if (c == '>') {
                    x1 = x1 + 1;
                } else if (c == 'v') {
                    y1 = y1 - 1;
                } else if (c == '<') {
                    x1 = x1 - 1;
                }
            } else {
                house.add(x2);
                house.add(y2);
                houses.add(house);
                if (c == '^') {
                    y2 = y2 + 1;
                } else if (c == '>') {
                    x2 = x2 + 1;
                } else if (c == 'v') {
                    y2 = y2 - 1;
                } else if (c == '<') {
                    x2 = x2 - 1;
                }
            }
            robot = !robot;
        }
        ArrayList<Integer> house = new ArrayList<Integer>();
        if (robot) {
            house.add(x1);
            house.add(y1);
        } else {
            house.add(x2);
            house.add(y2);
        }
        houses.add(house);
        System.out.println(houses.size());
    }
}