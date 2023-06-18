import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        ArrayList<ArrayList<Boolean>> lights = new ArrayList<ArrayList<Boolean>>();
        ArrayList<ArrayList<Boolean>> stuckLights = new ArrayList<ArrayList<Boolean>>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                ArrayList<Boolean> row = new ArrayList<Boolean>();
                ArrayList<Boolean> stuckRow = new ArrayList<Boolean>();
                for (char c : data.toCharArray()) {
                    if (c == '#') {
                        row.add(true);
                        stuckRow.add(true);
                    } else {
                        row.add(false);
                        stuckRow.add(false);
                    }
                }
                lights.add(row);
                stuckLights.add(stuckRow);
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        stuckLights.get(0).set(0, true);
        stuckLights.get(0).set(99, true);
        stuckLights.get(99).set(0, true);
        stuckLights.get(99).set(99, true);
        part1(lights);
        part2(stuckLights);
    }

    private static void part1(ArrayList<ArrayList<Boolean>> lights) {
        for (int i = 0; i < 100; i++){
            lights = nextGeneration(lights);
        }
        int sum = 0;
        for (ArrayList<Boolean> row : lights) {
            for (Boolean light : row) {
                if (light) {
                    sum++;
                }
            }
        }
        System.out.println("Part one: "+ sum);
    }

    private static ArrayList<ArrayList<Boolean>> nextGeneration(ArrayList<ArrayList<Boolean>> lights) {
        ArrayList<ArrayList<Boolean>> next = new ArrayList<ArrayList<Boolean>>();
        for (int i = 0; i < 100; i++) {
            ArrayList<Boolean> row = new ArrayList<Boolean>();
            for (int j = 0; j < 100; j++) {
                Boolean cur = lights.get(i).get(j);
                int neighbors = 0;
                if (i > 0 && lights.get(i-1).get(j)) { // up
                    neighbors++;
                }
                if (i > 0 && j > 0 && lights.get(i-1).get(j-1)) { //up-left
                    neighbors++;
                }
                if (j > 0 && lights.get(i).get(j-1)) { //left
                    neighbors++;
                }
                if (j > 0 && i < 99 && lights.get(i+1).get(j-1)) { // down-left
                    neighbors++;
                }
                if (i < 99 && lights.get(i+1).get(j)) { //down
                    neighbors++;
                }
                if (i < 99 && j < 99 && lights.get(i+1).get(j+1)) { //down-right
                    neighbors++;
                }
                if (j < 99 && lights.get(i).get(j+1)) { //right
                    neighbors++;
                }
                if (i > 0 && j < 99 && lights.get(i-1).get(j+1)) { //up-right
                    neighbors++;
                }
                if (cur && (neighbors < 2 || neighbors > 3)) {
                    row.add(false);
                } else if (cur && neighbors >= 2 && neighbors <= 3) {
                    row.add(true);
                } else if (!cur && neighbors == 3) {
                    row.add(true);
                } else if (!cur && neighbors != 3) {
                    row.add(false);
                }
            }
            next.add(row);
        }
        return next;
    }

    private static void part2(ArrayList<ArrayList<Boolean>> lights) {
        for (int i = 0; i < 100; i++){
            lights = nextGenerationStuck(lights);
        }
        int sum = 0;
        for (ArrayList<Boolean> row : lights) {
            for (Boolean light : row) {
                if (light) {
                    sum++;
                }
            }
        }
        System.out.println("Part two: "+ sum);
    }

    private static ArrayList<ArrayList<Boolean>> nextGenerationStuck(ArrayList<ArrayList<Boolean>> lights) {
        ArrayList<ArrayList<Boolean>> next = new ArrayList<ArrayList<Boolean>>();
        for (int i = 0; i < 100; i++) {
            ArrayList<Boolean> row = new ArrayList<Boolean>();
            for (int j = 0; j < 100; j++) {
                Boolean cur = lights.get(i).get(j);
                int neighbors = 0;
                if (i > 0 && lights.get(i-1).get(j)) { // up
                    neighbors++;
                }
                if (i > 0 && j > 0 && lights.get(i-1).get(j-1)) { //up-left
                    neighbors++;
                }
                if (j > 0 && lights.get(i).get(j-1)) { //left
                    neighbors++;
                }
                if (j > 0 && i < 99 && lights.get(i+1).get(j-1)) { // down-left
                    neighbors++;
                }
                if (i < 99 && lights.get(i+1).get(j)) { //down
                    neighbors++;
                }
                if (i < 99 && j < 99 && lights.get(i+1).get(j+1)) { //down-right
                    neighbors++;
                }
                if (j < 99 && lights.get(i).get(j+1)) { //right
                    neighbors++;
                }
                if (i > 0 && j < 99 && lights.get(i-1).get(j+1)) { //up-right
                    neighbors++;
                }
                if (cur && (neighbors < 2 || neighbors > 3)) {
                    row.add(false);
                } else if (cur && neighbors >= 2 && neighbors <= 3) {
                    row.add(true);
                } else if (!cur && neighbors == 3) {
                    row.add(true);
                } else if (!cur && neighbors != 3) {
                    row.add(false);
                }
            }
            next.add(row);
        }
        next.get(0).set(0, true);
        next.get(0).set(99, true);
        next.get(99).set(0, true);
        next.get(99).set(99, true);
        return next;
    }
}