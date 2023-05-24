import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class Main {
    public static void main(String[] args) {
        String input = "iwrupvqb";
        boolean p1done = false;
        boolean p2done = false;
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            Integer suffix = 1;
            while(true) {
                String cur = input + suffix;
                md.update(cur.getBytes());
                byte[] digest = md.digest();
                StringBuffer buff = new StringBuffer();
                for (int i = 0; i < digest.length; ++i) {
                    buff.append(Integer.toHexString((digest[i] & 0xFF) | 0x100).substring(1, 3));
                }
                String hash = buff.toString();
                if (!p1done && hash.substring(0, 5).toString().equals("00000")) {
                    System.out.println(suffix);
                    p1done = true;
                }
                if (!p2done && hash.substring(0, 6).toString().equals("000000")) {
                    System.out.println(suffix);
                    p2done = true;
                }
                if (p1done && p2done) {
                    break;
                }
                suffix++;
            }
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException(e);
        }
    }
}