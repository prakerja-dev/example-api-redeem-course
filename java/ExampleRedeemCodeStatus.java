import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.net.HttpURLConnection;
import java.net.URL;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.security.SignatureException;
import java.util.Formatter;

import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

public class Example {
  private static final String HMAC_SHA1_ALGORITHM = "HmacSHA1";

  private static String toHexString(byte[] bytes) {
    try (Formatter formatter = new Formatter()) {
      for (byte b : bytes) {
        formatter.format("%02x", b);
      }

      return formatter.toString();
    }
  }

  public static String calculateRFC2104HMAC(String data, String key)
      throws SignatureException, NoSuchAlgorithmException, InvalidKeyException {
    SecretKeySpec signingKey = new SecretKeySpec(key.getBytes(), HMAC_SHA1_ALGORITHM);
    Mac mac = Mac.getInstance(HMAC_SHA1_ALGORITHM);
    mac.init(signingKey);
    return toHexString(mac.doFinal(data.getBytes()));
  }

  public static void main(String[] args) throws Exception {
    String client_code = "";
    String redeem_code = "";
    String key = "";
    String endpoint = "/api/v1/integration/payment/redeem-code/status";
    String timestamp = String.valueOf(System.currentTimeMillis() / 1000L);
    String text = client_code + timestamp + "POST" + endpoint + "{\"redeem_code\":\"" + redeem_code + "\"}";
    String signature = calculateRFC2104HMAC(text, key);

    System.out.println("timestamp: " + timestamp);
    System.out.println("signature: " + signature);

    URL url = new URL("https://api-ssn.prakerja.go.id" + endpoint);
    HttpURLConnection conn = (HttpURLConnection) url.openConnection();
    conn.setDoOutput(true);
    conn.setRequestMethod("POST");

    conn.setRequestProperty("Content-Type", "application/json");
    conn.setRequestProperty("client_code", client_code);
    conn.setRequestProperty("signature", signature);
    conn.setRequestProperty("timestamp", timestamp);

    String data = "{\"redeem_code\":\"" + redeem_code + "\"}";

    OutputStream os = conn.getOutputStream();
    os.write(data.getBytes());
    os.flush();

    BufferedReader br = new BufferedReader(new InputStreamReader(
        (conn.getInputStream())));

    String result;
    System.out.println("result: ");
    while ((result = br.readLine()) != null) {
      System.out.println(result);
    }

    conn.disconnect();
  }
}