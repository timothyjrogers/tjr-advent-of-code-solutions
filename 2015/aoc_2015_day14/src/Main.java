import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        ArrayList<Reindeer> reindeer = new ArrayList<Reindeer>();
        HashMap<String, Integer> scoreboard = new HashMap<String, Integer>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                String[] parts = data.split(" ");
                String name = parts[0];
                Integer speed = Integer.parseInt(parts[3]);
                Integer duration = Integer.parseInt(parts[6]);
                Integer rest = Integer.parseInt(parts[13]);
                Reindeer r = new Reindeer(speed, duration, rest, name);
                reindeer.add(r);
                scoreboard.put(name, 0);
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        for (int i = 0; i < 2503; i++) {
            HashMap<String, Integer> scores = new HashMap<String, Integer>();
            for (Reindeer r : reindeer) {
                r.tick();
                scores.put(r.getName(), r.getTravelled());
            }
            String winner = "";
            Integer max = 0;
            for (Map.Entry<String, Integer> current : scores.entrySet()) {
                if (current.getValue() > max) {
                    max = current.getValue();
                    winner = current.getKey();
                }
            }
            scoreboard.put(winner, scoreboard.get(winner)+1);
        }

        int max = 0;
        for (Reindeer r : reindeer) {
            if (r.getTravelled() > max) {
                max = r.getTravelled();
            }
        }
        System.out.println("Part one: " + max);
        Integer maxScore = 0;
        for (Map.Entry<String, Integer> current : scoreboard.entrySet()) {
            if (current.getValue() > maxScore) {
                maxScore = current.getValue();
            }
        }
        System.out.println("Part two: " + maxScore);
    }
}