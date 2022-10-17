from hashlib import sha1
import hmac

# if using this value bellow should output 943c20a8e36b6243f576404b00fffdc86411ce1f
text = 'myclient_code1516239022POST/api/v1/integration/payment/redeem-code/commit{"redeem_code":"code"}'
key = 'my_sign_key'

result = hmac.new(key.encode("utf-8"), text.encode("utf-8"), sha1).hexdigest()

print(result)
