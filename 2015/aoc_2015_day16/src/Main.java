import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.String;
import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        HashMap<String, Integer> mfcsam = new HashMap<String, Integer>();
        mfcsam.put("children", 3);
        mfcsam.put("cats", 7);
        mfcsam.put("samoyeds", 2);
        mfcsam.put("pomeranians", 3);
        mfcsam.put("akitas", 0);
        mfcsam.put("vizslas", 0);
        mfcsam.put("goldfish", 5);
        mfcsam.put("trees", 3);
        mfcsam.put("cars", 2);
        mfcsam.put("perfumes", 1);
        int maxMatches = 0;
        String bestSue = "";
        int maxMatches_p2 = 0;
        String bestSue_p2 = "";
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                String sue = data.substring(data.indexOf(":")+2);
                String[] details = sue.split(", ");
                int matches = 0;
                int matches_p2 = 0;
                for (String detail : details) {
                    String[] parts = detail.split(": ");
                    int amount = Integer.parseInt(parts[1]);
                    if (mfcsam.get(parts[0]) == amount) {
                        matches++;
                    }
                    if ((parts[0].equals("cats") || parts[0].equals("trees")) && mfcsam.get(parts[0]) < amount) {
                        matches_p2++;
                    } else if ((parts[0].equals("pomeranians") || parts[0].equals("goldfish")) && mfcsam.get(parts[0]) > amount) {
                        matches_p2++;
                    } else if ((!parts[0].equals("cats") && !parts[0].equals("trees") && !parts[0].equals("pomeranians") && !parts[0].equals("goldfish")) && mfcsam.get(parts[0]) == amount) {
                        matches_p2++;
                    }
                }
                if (matches > maxMatches) {
                    maxMatches = matches;
                    bestSue = data;
                }
                if (matches_p2 > maxMatches_p2) {
                    maxMatches_p2 = matches_p2;
                    bestSue_p2 = data;
                }
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        System.out.println(bestSue);
        System.out.println(bestSue_p2);
    }
}