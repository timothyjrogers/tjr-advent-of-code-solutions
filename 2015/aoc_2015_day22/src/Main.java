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
        int player_health = 50;
        int player_mana = 500;
        GameState start = new GameState(player_health, player_mana, boss_hp, boss_dmg, false);
        System.out.println("Part one: " + simulate(start));
        start = new GameState(player_health, player_mana, boss_hp, boss_dmg, true);
        System.out.println("Part two: " + simulate(start));
    }

    private static int simulate(GameState initial) {
        PriorityQueue<GameState> states = new PriorityQueue<>((a,b) -> Integer.compare(a.getManaSpent(), b.getManaSpent()));
        states.add(new GameState(initial));
        int minCost = Integer.MAX_VALUE;

        while (!states.isEmpty()) {
            GameState current = states.poll();
            if (current.getManaSpent() > minCost) {
                continue;
            }
            if (current.gameOver() == 1) {
                if (current.getManaSpent() < minCost) {
                    minCost = current.getManaSpent();
                }
                continue;
            } else if (current.gameOver() == -1) {
                continue;
            }

            if (current.isMagicMissileAvailable()) {
                GameState next = current.processTurn(GameState.PlayerActions.MAGICMISSILE);
                states.add(next);
            }
            if (current.isDrainAvailable()) {
                GameState next = current.processTurn(GameState.PlayerActions.DRAIN);
                states.add(next);
            }
            if (current.isShieldAvailable()) {
                GameState next = current.processTurn(GameState.PlayerActions.SHIELD);
                states.add(next);
            }
            if (current.isPoisonAvailable()) {
                GameState next = current.processTurn(GameState.PlayerActions.POISON);
                states.add(next);
            }
            if (current.isRechargeAvailable()) {
                GameState next = current.processTurn(GameState.PlayerActions.RECHARGE);
                states.add(next);
            }
        }
        return minCost;
    }
}