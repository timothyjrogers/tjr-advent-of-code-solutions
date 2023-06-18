import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.lang.String;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        final String inputFile = "input.txt";
        String data = "";
        BufferedReader reader;
        ArrayList<Ingredient> recipe = new ArrayList<Ingredient>();
        try {
            reader = new BufferedReader(new FileReader(inputFile));
            data = reader.readLine();
            while (data != null) {
                String[] props = data.split(": ")[1].split(", ");
                int capacity = Integer.parseInt(props[0].split(" ")[1]);
                int durability = Integer.parseInt(props[1].split(" ")[1]);
                int flavor = Integer.parseInt(props[2].split(" ")[1]);
                int texture = Integer.parseInt(props[3].split(" ")[1]);
                int calories = Integer.parseInt(props[4].split(" ")[1]);
                Ingredient ingredient = new Ingredient(capacity, durability, flavor, texture, calories);
                recipe.add(ingredient);
                data = reader.readLine();
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        part1(recipe);
        part2(recipe);
    }

    private static void part1(ArrayList<Ingredient> recipe) {
        int max = 0;
        for (int i = 0; i <= 100; i++) {
            for (int j = 0; j <= 100 - i; j++) {
                for  (int k = 0; k <= 100 - i - j; k++) {
                    int l = 100 - i -j - k;
                    int capacity = Math.max(0, i*recipe.get(0).getCapacity() + j*recipe.get(1).getCapacity() + k*recipe.get(2).getCapacity() + l*recipe.get(3).getCapacity());
                    int durability = Math.max(0, i*recipe.get(0).getDurability() + j*recipe.get(1).getDurability() + k*recipe.get(2).getDurability() + l*recipe.get(3).getDurability());
                    int flavor = Math.max(0, i*recipe.get(0).getFlavor() + j*recipe.get(1).getFlavor() + k*recipe.get(2).getFlavor() + l*recipe.get(3).getFlavor());
                    int texture = Math.max(0, i*recipe.get(0).getTexture() + j*recipe.get(1).getTexture() + k*recipe.get(2).getTexture() + l*recipe.get(3).getTexture());
                    if (capacity * durability * flavor * texture > max) {
                        max = capacity * durability * flavor * texture;
                    }
                }
            }
        }
        System.out.println("Part one: " + max);
    }

    private static void part2(ArrayList<Ingredient> recipe) {
        int max = 0;
        for (int i = 0; i <= 100; i++) {
            for (int j = 0; j <= 100 - i; j++) {
                for  (int k = 0; k <= 100 - i - j; k++) {
                    int l = 100 - i - j - k;
                    int capacity = Math.max(0, i*recipe.get(0).getCapacity() + j*recipe.get(1).getCapacity() + k*recipe.get(2).getCapacity() + l*recipe.get(3).getCapacity());
                    int durability = Math.max(0, i*recipe.get(0).getDurability() + j*recipe.get(1).getDurability() + k*recipe.get(2).getDurability() + l*recipe.get(3).getDurability());
                    int flavor = Math.max(0, i*recipe.get(0).getFlavor() + j*recipe.get(1).getFlavor() + k*recipe.get(2).getFlavor() + l*recipe.get(3).getFlavor());
                    int texture = Math.max(0, i*recipe.get(0).getTexture() + j*recipe.get(1).getTexture() + k*recipe.get(2).getTexture() + l*recipe.get(3).getTexture());
                    int calories = i*recipe.get(0).getCalories() + j*recipe.get(1).getCalories() + k*recipe.get(2).getCalories() + l*recipe.get(3).getCalories();
                    if (calories == 500 && capacity * durability * flavor * texture > max) {
                        max = capacity * durability * flavor * texture;
                    }
                }
            }
        }
        System.out.println("Part two: " + max);
    }
}