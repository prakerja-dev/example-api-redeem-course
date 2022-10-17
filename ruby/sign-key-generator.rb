require 'base64'
require 'cgi'
require 'openssl'

# if using this value bellow should output 943c20a8e36b6243f576404b00fffdc86411ce1f
text = 'myclient_code1516239022POST/api/v1/integration/payment/redeem-code/commit{"redeem_code":"code"}'
key = 'my_sign_key'

hmac = OpenSSL::HMAC.hexdigest(OpenSSL::Digest::Digest.new('sha1'), key.encode("ASCII"), text.encode("ASCII"))

puts hmac