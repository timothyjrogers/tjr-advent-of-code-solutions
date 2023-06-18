public class Item {
    int cost;
    int damage;
    int armor;

    public Item(int cost, int damage, int armor) {
        this.cost = cost;
        this.damage = damage;
        this.armor = armor;
    }

    public int getCost() {
        return this.cost;
    }

    public int getDamage() {
        return this.damage;
    }

    public int getArmor() {
        return this.armor;
    }
}
