import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
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
        char[] steps = data.toCharArray();
        part1(steps);
        part2(steps);
    }

    private static void part1(char[] steps) {
        int result = 0;
        for(char c : steps) {
            if (c == '(') {
                result = result + 1;
            } else if (c == ')') {
                result = result - 1;
            }
        }
        System.out.println(result);
    }

    private static void part2(char[] steps) {
        int result = 0;
        for (int i = 0; i < steps.length; i++) {
            char c = steps[i];
            if (c == '(') {
                result = result + 1;
            } else if (c == ')') {
                result = result - 1;
            }
            if (result < 0) {
                System.out.println(i+1);
                break;
            }
        }
    }
}