public class Ingredient {
    int capacity;
    int durability;
    int flavor;
    int texture;
    int calories;

    public Ingredient(int capacity, int durability, int flavor, int texture, int calories) {
        this.capacity = capacity;
        this.durability = durability;
        this.flavor = flavor;
        this.texture = texture;
        this.calories = calories;
    }

    public int getCapacity() {
        return this.capacity;
    }

    public int getDurability() {
        return this.durability;
    }

    public int getFlavor() {
        return this.flavor;
    }

    public int getTexture() {
        return this.texture;
    }

    public int getCalories() {
        return this.calories;
    }
}
