import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        int boss_hp = 0;
        int boss_dmg = 0;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            boss_hp = Integer.parseInt(data.split(": ")[1]);
            data = reader.readLine();
            boss_dmg = Integer.parseInt(data.split(": ")[1]);
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        HashMap<String, Integer> effects = new HashMap<String, Integer>();
        effects.put("Shield", 0);
        effects.put("Poison", 0);
        effects.put("Recharge", 0);

    }
}