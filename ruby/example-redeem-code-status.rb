require 'base64'
require 'cgi'
require 'openssl'
require 'net/http'
require 'uri'
require 'json'

client_code = ''
redeem_code = ''
key = ''
endpoint = '/api/v1/integration/payment/redeem-code/status'
timestamp = String(Time.now.to_i)
text = client_code + timestamp + 'POST' + endpoint + '{"redeem_code":"' + redeem_code + '"}'
signature = OpenSSL::HMAC.hexdigest(OpenSSL::Digest::Digest.new('sha1'), key.encode("ASCII"), text.encode("ASCII"))

puts 'timestamp: ' + timestamp
puts 'signature: ' + signature

data = {'redeem_code' => redeem_code}

uri = URI.parse('https://api-ssn.prakerja.go.id' + endpoint)
request = Net::HTTP::Post.new(uri)
request.content_type = "application/json"
request.body = JSON.generate(data, quirks_mode: true)
request['Client_code'] = client_code
request['Signature'] = signature
request['Timestamp'] = timestamp

req_options = {
  use_ssl: uri.scheme == "https",
}

result = Net::HTTP.start(uri.hostname, uri.port, req_options) do |http|
  http.request(request)
end

# result.code
# result.body
puts 'result: ' + result.body