public class Reindeer {
    int speed;
    int duration;
    int rest;
    boolean resting;
    int restedTime = 0;
    int travelledTime = 0;
    int travelled = 0;
    String name;
    public Reindeer(int speed, int duration, int rest, String name) {
        this.speed = speed;
        this.duration = duration;
        this.rest = rest;
        this.resting = false;
        this.travelledTime = 0;
        this.restedTime = 0;
        this.travelled = 0;
        this.name = name;
    }

    public void tick() {
        if (this.resting) {
            this.restedTime++;
            if (this.restedTime == this.rest) {
                this.restedTime = 0;
                this.resting = false;
            }
            return;
        }
        this.travelled += this.speed;
        this.travelledTime++;
        if (this.travelledTime == this.duration) {
            this.resting = true;
            this.travelledTime = 0;
        }
    }

    public int getTravelled() {
        return this.travelled;
    }

    public String getName() {
        return this.name;
    }
}
