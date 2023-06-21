import java.util.HashMap;

public class GameState {
    int player_health;
    int player_mana;
    int player_armor;
    int boss_health;
    int boss_damage;
    int mana_spent;

    String winner;

    boolean hard;

    HashMap<String, Integer> effects;

    public enum PlayerActions {
        MAGICMISSILE,
        DRAIN,
        SHIELD,
        POISON,
        RECHARGE
    }
    public GameState(int player_health, int player_mana, int boss_health, int boss_damage, boolean hard) {
        this.player_health = player_health;
        this.player_mana = player_mana;
        this.player_armor = 0;
        this.boss_health = boss_health;
        this.boss_damage = boss_damage;
        this.mana_spent = 0;
        this.winner = "";
        this.hard = hard;
        this.effects = new HashMap<>();
    }

    public GameState(GameState orig) {
        this.player_health = orig.getPlayerHealth();
        this.player_mana = orig.getPlayerMana();
        this.player_armor = orig.getPlayerArmor();
        this.boss_health = orig.getBossHealth();
        this.boss_damage = orig.getBossDamage();
        this.mana_spent = orig.getManaSpent();
        this.winner = orig.getWinner();
        this.hard = orig.getHard();
        this.effects = new HashMap<>();
        orig.getEffects().forEach((k,v) -> {
            this.effects.put(k, v);
        });
    }

    public int getPlayerHealth() {
        return this.player_health;
    }
    public int getPlayerMana() {
        return this.player_mana;
    }

    public int getPlayerArmor() {
        return this.player_armor;
    }

    public int getBossHealth() {
        return this.boss_health;
    }

    public int getBossDamage() {
        return this.boss_damage;
    }

    public int getManaSpent() {
        return this.mana_spent;
    }

    public String getWinner() {
        return this.winner;
    }

    public void setWinner(String winner) {
        this.winner = winner;
    }

    public boolean getHard() {
        return this.hard;
    }

    public HashMap<String, Integer> getEffects() {
        return this.effects;
    }
    public int gameOver() {
        if (this.boss_health < 1) {
            return 1;
        } else if (this.player_health < 1) {
            return -1;
        } else {
            return 0;
        }
    }

    public boolean isMagicMissileAvailable() {
        return this.player_mana >= 53;
    }

    public boolean isDrainAvailable() {
        return this.player_mana >= 73;
    }
    public boolean isShieldAvailable() {
        if (this.effects.containsKey("Shield")) {
            return this.effects.get("Shield") == 1 && this.player_mana >= 113;
        }
        return this.player_mana >= 113;
    }

    public boolean isPoisonAvailable() {
        if (this.effects.containsKey("Poison")) {
            return this.effects.get("Poison") == 1 && this.player_mana >= 173;
        }
        return this.player_mana >= 173;
    }

    public boolean isRechargeAvailable() {
        if (this.effects.containsKey("Recharge")) {
            return this.effects.get("Recharge") == 1 && this.player_mana >= 229;
        }
        return this.player_mana >= 229;
    }

    public void handleEffects() {
        if (this.effects.containsKey("Poison")) {
            this.boss_health -= 3;
            int duration = this.effects.get("Poison") - 1;
            if (duration > 0) {
                this.effects.put("Poison", duration);
            } else {
                this.effects.remove("Poison");
            }
        }
        if (this.effects.containsKey("Shield")) {
            int duration = this.effects.get("Shield") - 1;
            if (duration < 1) {
                this.player_armor = 0;
                this.effects.remove("Shield");
            } else {
                this.effects.put("Shield", duration);
            }
        }
        if (this.effects.containsKey("Recharge")) {
            this.player_mana += 101;
            int duration = this.effects.get("Recharge") - 1;
            if (duration > 0) {
                this.effects.put("Recharge", duration);
            } else {
                this.effects.remove("Recharge");
            }
        }
    }

    public void bossTurn() {
        this.player_health -= Math.max(1, this.boss_damage - this.player_armor);
    }

    public void doAction(PlayerActions action) {
        switch(action) {
            case MAGICMISSILE:
                this.mana_spent += 53;
                this.player_mana -= 53;
                this.boss_health -= 4;
                break;
            case DRAIN:
                this.mana_spent += 73;
                this.player_mana -= 73;
                this.boss_health -= 2;
                this.player_health += 2;
                break;
            case SHIELD:
                    this.effects.put("Shield", 6);
                    this.mana_spent += 113;
                    this.player_mana -= 113;
                    this.player_armor = 7;
                break;
            case POISON:
                    this.effects.put("Poison", 6);
                    this.mana_spent += 173;
                    this.player_mana -= 173;
                break;
            case RECHARGE:
                    this.effects.put("Recharge", 5);
                    this.mana_spent += 229;
                    this.player_mana -= 229;
                break;
            default:
                break;
        }
    }
    public void decrementPlayerHealth() {
            this.player_health--;
    }
    public GameState processTurn(PlayerActions action) {
        GameState next = new GameState(this);
        if (next.getHard()) {
            next.decrementPlayerHealth();
            if (next.getBossHealth() <= 0) {
                next.setWinner("p");
                return next;
            }
        }
        next.handleEffects();
        if (next.getBossHealth() <= 0) {
            next.setWinner("p");
            return next;
        } else if (next.getPlayerMana() < 53) {
            next.setWinner("b");
            return next;
        }
        next.doAction(action);
        if (next.getBossHealth() <= 0) {
            next.setWinner("p");
            return next;
        }
        next.handleEffects();
        if (next.getBossHealth() <= 0) {
            next.setWinner("p");
            return next;
        }
        next.bossTurn();
        if (next.getPlayerHealth() <= 0) {
            next.setWinner("b");
            return next;
        }
        return next;
    }

}
