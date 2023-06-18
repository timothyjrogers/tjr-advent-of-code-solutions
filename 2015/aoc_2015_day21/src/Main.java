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
        int boss_armor = 0;
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            boss_hp = Integer.parseInt(data.split(": ")[1]);
            data = reader.readLine();
            boss_dmg = Integer.parseInt(data.split(": ")[1]);
            data = reader.readLine();
            boss_armor = Integer.parseInt(data.split(": ")[1]);
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        ArrayList<Item> weapons = new ArrayList<>();
        weapons.add(new Item(8, 4, 0));
        weapons.add(new Item(10, 5, 0));
        weapons.add(new Item(25, 6, 0));
        weapons.add(new Item(40, 7, 0));
        weapons.add(new Item(74, 8, 0));
        ArrayList<Item> armors = new ArrayList<>();
        armors.add(new Item(13, 0, 1));
        armors.add(new Item(31, 0, 2));
        armors.add(new Item(53, 0, 3));
        armors.add(new Item(75, 0, 4));
        armors.add(new Item(102, 0, 5));
        ArrayList<Item> rings = new ArrayList<>();
        rings.add(new Item(25, 1, 0));
        rings.add(new Item(50, 2, 0));
        rings.add(new Item(100, 3, 0));
        rings.add(new Item(20, 0, 1));
        rings.add(new Item(40, 0, 2));
        rings.add(new Item(80, 0, 3));

        ArrayList<Item> combinations = new ArrayList<>();
        for (int i = 0; i < weapons.size(); i++) {
            int cost = weapons.get(i).getCost();
            int damage = weapons.get(i).getDamage();
            int armor = weapons.get(i).getArmor();
            combinations.add(new Item(cost, damage, armor));
            for (int j = 0; j < armors.size(); j++) {
                int acost = armors.get(j).getCost();
                int adamage = armors.get(j).getDamage();
                int aarmor = armors.get(j).getArmor();
                combinations.add(new Item(cost + acost, damage + adamage, armor + aarmor));
                for (int k = 0; k < rings.size(); k++) {
                    int rcost = rings.get(k).getCost();
                    int rdamage = rings.get(k).getDamage();
                    int rarmor = rings.get(k).getArmor();
                    combinations.add(new Item(cost + acost + rcost, damage + adamage + rdamage, armor + aarmor + rarmor));
                }
                for (int k = 0; k < rings.size()-1; k++) {
                    for (int l = 0; l < rings.size(); l++) {
                        int rcost = rings.get(k).getCost() + rings.get(l).getCost();
                        int rdamage = rings.get(k).getDamage() + rings.get(l).getDamage();
                        int rarmor = rings.get(k).getArmor() + rings.get(l).getArmor();
                        combinations.add(new Item(cost + acost + rcost, damage + adamage + rdamage, armor + aarmor + rarmor));
                    }
                }
            }
            for (int j = 0; j < rings.size(); j++) {
                int rcost = rings.get(j).getCost();
                int rdamage = rings.get(j).getDamage();
                int rarmor = rings.get(j).getArmor();
                combinations.add(new Item(cost + rcost, damage + rdamage, armor + rarmor));
            }
            for (int j = 0; j < rings.size()-1; j++) {
                for (int k = 0; k < rings.size(); k++) {
                    int rcost = rings.get(j).getCost() + rings.get(k).getCost();
                    int rdamage = rings.get(j).getDamage() + rings.get(k).getDamage();
                    int rarmor = rings.get(j).getArmor() + rings.get(k).getArmor();
                    combinations.add(new Item(cost + rcost, damage + rdamage, armor + rarmor));
                }
            }
        }
        int minCost = Integer.MAX_VALUE;
        int maxCost = 0;
        for (Item item : combinations) {
            int phealth = 100;
            int bhealth = boss_hp;
            boolean pturn = true;
            while (phealth > 0 && bhealth > 0) {
                if (pturn) {
                    bhealth = bhealth - Math.max(1, item.getDamage() - boss_armor);
                    pturn = false;
                } else {
                    phealth = phealth - Math.max(1, boss_dmg - item.getArmor());
                    pturn = true;
                }
            }
            if (phealth > 0 && item.getCost() < minCost) {
                minCost = item.getCost();
            }
            if (bhealth > 0 && item.getCost() > maxCost) {
                maxCost = item.getCost();
            }
        }
        System.out.println("Part one: " + minCost);
        System.out.println("Part two: " + maxCost);
    }
}