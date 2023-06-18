public class Main {
    public static void main(String[] args) {
        String input = "cqjxjnds";
        int count = 0;
        while (count < 2) {
            if (isValid(input)) {
                System.out.println(input);
                count++;
            }
            input = incrementBaseAlphabet(input);
        }
    }

    private static String incrementBaseAlphabet(String input) {
        int maxAlpha = 122;
        int minAlpha = 97;
        char[] chars = input.toCharArray();
        boolean carry = true;
        for (int i = input.length() - 1; i >= 0; i--) {
            int c = (int)chars[i];
            if (carry) {
                c++;
                carry = false;
            } else {
                break;
            }
            if (c > maxAlpha) {
                c = minAlpha;
                carry = true;
            } else if (c == (int)'i' || c == (int)'l' || c == (int)'o') {
                c++;
            }
            chars[i] = (char)c;
        }
        return String.valueOf(chars);
    }

    private static boolean isValid(String s) {
        if (s.contains("i") || s.contains("l") || s.contains("o")) {
            return false;
        }
        char[] chars = s.toCharArray();
        boolean triplet = false;
        int pairs = 0;
        for (int i = 0; i < chars.length-3; i++) {
            if (chars[i+2] - chars[i+1] == 1 && chars[i+1] - chars[i] == 1) {
                triplet = true;
                break;
            }
        }
        if (!triplet) {
            return false;
        }
        for (int i = 0; i < chars.length-1; i++) {
            if (chars[i] == chars[i+1]) {
                pairs++;
                i++;
            }
        }
        return triplet && pairs >= 2;
    }
}