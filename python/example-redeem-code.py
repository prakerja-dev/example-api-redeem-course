from hashlib import sha1
import hmac
import time
import requests
import json

client_code = ''
redeem_code = ''
key = ''
endpoint = '/api/v1/integration/payment/redeem-code/status'
timestamp = str(int(time.time()))
text = client_code + timestamp + 'POST' + \
    endpoint + '{"redeem_code":"' + redeem_code + '"}'
signature = hmac.new(key.encode("utf-8"),
                     text.encode("utf-8"), sha1).hexdigest()

print('timestamp: ' + timestamp)
print('signature: ' + signature)

headers = {
    'Content-Type': 'application/json',
    'client_code': client_code,
    'signature': signature,
    'timestamp': timestamp
}
data = {
    'redeem_code': redeem_code
}

result = requests.post(
    'https://api-ssn.prakerja.go.id' + endpoint,
    headers=headers,
    data=json.dumps(data)
)

print('result: ')
print(result.text)
