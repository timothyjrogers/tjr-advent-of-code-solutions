public class Main {
    public static void main(String[] args) {
        String input = "1321131112";
        for (int i = 0; i < 50; i++) {
            if (i == 40) {
                System.out.println("Part 1: " + input.length());
            }
            StringBuilder next = new StringBuilder();
            char[] chars = input.toCharArray();
            int counter = 0;
            char current = chars[0];
            for (int j = 0; j < chars.length; j++) {
                if (chars[j] == current) {
                    counter++;
                } else {
                    next.append(counter);
                    next.append(current);
                    current = chars[j];
                    counter = 1;
                }
            }
            next.append(counter);
            next.append(current);
            input = next.toString();
        }
        System.out.println("Part 2: " + input.length());
    }
}