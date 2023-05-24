public class Box {
    int length;
    int width;
    int height;
    public Box(int length, int width, int height) {
        this.length = length;
        this.width = width;
        this.height = height;
    }

    public int getTotalPaper() {
        return this.area() + this.smallestSide();
    }

    private int area() {
        return 2*this.length*this.width + 2*this.length*this.height + 2*this.width*this.height;
    }

    private int smallestSide() {
        return Math.min(this.length*this.width, Math.min(this.length*this.height, this.width*this.height));
    }

    public int getVolume() {
        return this.length * this.width * this.height;
    }
    public int getRequiredLength() {
        int a = 2 * this.length + 2 * this.width;
        int b = 2 * this.width + 2 * this.height;
        int c = 2 * this.length + 2 * this.height;
        return Math.min(a, Math.min(b, c));
    }
}
