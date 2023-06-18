import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";

        String data = "";
        BufferedReader reader;
        int raw_length = 0;
        int mem_length = 0;
        int encoded_length = 0;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                char[] chars = data.toCharArray();
                for (int i = 0; i < data.length() - 1; i++) {
                    raw_length++;
                    mem_length++;
                    encoded_length++;
                    if (chars[i] == '\\') {
                        if (chars[i+1] == '\\') {
                            raw_length++;
                            encoded_length = encoded_length + 3;
                            i++;
                        } else if (chars[i+1] == '"') {
                            raw_length++;
                            encoded_length = encoded_length + 3;
                            i++;
                        } else {
                            raw_length = raw_length + 3;
                            encoded_length = encoded_length + 4;
                            i = i + 3;
                        }
                    }
                }
                mem_length = mem_length - 1;
                raw_length++;
                encoded_length = encoded_length + 5;
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        System.out.println("Part 1: " + raw_length + " - " + mem_length + " = " + (raw_length - mem_length));
        System.out.println("Part 2: " + encoded_length + " - " + raw_length + " = " + (encoded_length - raw_length));
    }
}