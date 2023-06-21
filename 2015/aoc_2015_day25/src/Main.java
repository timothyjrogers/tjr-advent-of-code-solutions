import com.sun.jdi.IntegerValue;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.*;

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
        int targetRow = Integer.parseInt(data.split("row ")[1].split(",")[0]);
        int targetColumn = Integer.parseInt(data.split("column ")[1].substring(0, 4));
        ArrayList<ArrayList<Long>> paper = new ArrayList<>();
        int x = 0;
        int y  = 0;
        long latest = 20151125;
        while (true) {
            if (paper.size() > targetRow && paper.get(targetRow).size() > targetColumn) {
                break;
            }
            if (paper.size() == y) {
                ArrayList<Long> newRow = new ArrayList<>();
                paper.add(newRow);
            }
            paper.get(y).add(latest);
            if (paper.get(0).size() == paper.size()) {
                x = 0;
                y = paper.size();
            } else {
                x++;
                y--;
            }
            latest = (latest * 252533) % 33554393;
        }
        System.out.println("Part one: " + paper.get(targetRow-1).get(targetColumn-1));
    }
}