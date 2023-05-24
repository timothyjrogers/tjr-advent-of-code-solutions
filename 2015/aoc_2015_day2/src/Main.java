import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.String;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        int solution1 = 0;
        int solution2 = 0;
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                String[] dimensions = data.split("x");
                Box b = new Box(Integer.parseInt(dimensions[0]), Integer.parseInt(dimensions[1]), Integer.parseInt(dimensions[2]));
                solution1 = solution1 + b.getTotalPaper();
                solution2 = solution2 + b.getRequiredLength() + b.getVolume();
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        System.out.println(solution1);
        System.out.println(solution2);
    }
}