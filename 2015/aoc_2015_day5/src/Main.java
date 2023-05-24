import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        part1(inputFile);
        part2(inputFile);
    }

    private static void part1(String inputFile) {
        String data = "";
        BufferedReader reader;
        int nice = 0;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                char[] letters = data.toCharArray();
                int vowels = 0;
                boolean repeat = false;
                boolean usesForbidden = false;
                ArrayList<String> forbidden = new ArrayList<String>();
                forbidden.add("ab");
                forbidden.add("cd");
                forbidden.add("pq");
                forbidden.add("xy");
                if (letters[0] == 'a' || letters[0] == 'e' || letters[0] == 'i' || letters[0] == 'o' || letters[0] == 'u') {
                    vowels++;
                }
                for (int i = 0; i < letters.length - 1; i++) {
                    if (letters[i + 1] == 'a' || letters[i + 1] == 'e' || letters[i + 1] == 'i' || letters[i + 1] == 'o' || letters[i + 1] == 'u') {
                        vowels++;
                    }
                    if (letters[i] == letters[i + 1]) {
                        repeat = true;
                    }
                    StringBuilder builder = new StringBuilder();
                    builder.append(letters[i]);
                    builder.append(letters[i + 1]);
                    if (forbidden.contains(builder.toString())) {
                        usesForbidden = true;
                    }
                }
                if (vowels >= 3 && repeat && !usesForbidden) {
                    nice++;
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        System.out.println(nice );
    }

    private static void part2(String inputFile) {
        String data = "";
        BufferedReader reader;
        int nice = 0;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                char[] letters = data.toCharArray();
                boolean duplicatePair = false;
                boolean sandwich = false;
                for (int i = 0; i < letters.length - 1; i++) {
                    if (i >= 1 && i < letters.length - 1 && letters[i-1] == letters[i+1]) {
                        sandwich = true;
                    }
                    if (!duplicatePair) {
                        for (int j = i + 2; j < letters.length - 1; j++) {
                            if (letters[i] == letters[j] && letters[i + 1] == letters[j + 1]) {
                                duplicatePair = true;
                                break;
                            }
                        }
                    }
                }
                if (duplicatePair && sandwich) {
                    nice++;
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        System.out.println(nice);
    }
}