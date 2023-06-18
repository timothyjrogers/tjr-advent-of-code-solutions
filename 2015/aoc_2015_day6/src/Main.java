import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        part1(inputFile);
        part2(inputFile);
    }

    private static void part1(String inputFile) {
        String data = "";
        BufferedReader reader;

        boolean[][] lights = new boolean[1000][1000];

        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                ArrayList<String> parts = new ArrayList<String>();
                for (String s : data.split(",")) {
                    for (String q : s.split(" ")) {
                        parts.add(q);
                    }
                }
                Integer x1 = data.startsWith("turn") ? Integer.parseInt(parts.get(2)) : Integer.parseInt(parts.get(1));
                Integer y1 = data.startsWith("turn") ? Integer.parseInt(parts.get(3)) : Integer.parseInt(parts.get(2));
                Integer x2 = data.startsWith("turn") ? Integer.parseInt(parts.get(5)) : Integer.parseInt(parts.get(4));
                Integer y2 = data.startsWith("turn") ? Integer.parseInt(parts.get(6)) : Integer.parseInt(parts.get(5));
                for (int i = y1; i <= y2; i++) {
                    for (int j = x1; j <= x2; j++) {
                        if (data.startsWith("turn on")) {
                            lights[i][j] = true;
                        } else if (data.startsWith("turn off")) {
                            lights[i][j] = false;
                        } else {
                            lights[i][j] = !lights[i][j];
                        }
                    }
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        int on = 0;
        for (int i = 0; i < 1000; i++) {
            for (int j = 0; j < 1000; j++) {
                if (lights[i][j]) {
                    on++;
                }
            }
        }
        System.out.println(on);
    }

    private static void part2(String inputFile) {
        String data = "";
        BufferedReader reader;

        int[][] lights = new int[1000][1000];

        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                ArrayList<String> parts = new ArrayList<String>();
                for (String s : data.split(",")) {
                    for (String q : s.split(" ")) {
                        parts.add(q);
                    }
                }
                Integer x1 = data.startsWith("turn") ? Integer.parseInt(parts.get(2)) : Integer.parseInt(parts.get(1));
                Integer y1 = data.startsWith("turn") ? Integer.parseInt(parts.get(3)) : Integer.parseInt(parts.get(2));
                Integer x2 = data.startsWith("turn") ? Integer.parseInt(parts.get(5)) : Integer.parseInt(parts.get(4));
                Integer y2 = data.startsWith("turn") ? Integer.parseInt(parts.get(6)) : Integer.parseInt(parts.get(5));
                for (int i = y1; i <= y2; i++) {
                    for (int j = x1; j <= x2; j++) {
                        if (data.startsWith("turn on")) {
                            lights[i][j]++;
                        } else if (data.startsWith("turn off")) {
                            lights[i][j] = lights[i][j] > 1 ? lights[i][j] - 1 : 0;
                        } else {
                            lights[i][j] = lights[i][j] + 2;
                        }
                    }
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        int brightness = 0;
        for (int i = 0; i < 1000; i++) {
            for (int j = 0; j < 1000; j++) {
                brightness = brightness + lights[i][j];
            }
        }
        System.out.println(brightness);
    }
}