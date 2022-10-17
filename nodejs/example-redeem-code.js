const axios = require('axios');
const crypto = require('crypto-js');

const client_code = '';
const redeem_code = '';
const key = '';
const endpoint = '/api/v1/integration/payment/redeem-code/status';
const timestamp = Math.round((new Date()).getTime() / 1000);
const text = client_code + timestamp + 'POST' + endpoint + '{"redeem_code":"' + redeem_code + '"}';
const signature = crypto.HmacSHA1(text, key).toString(crypto.enc.Hex);

console.log('timestamp: ', timestamp);
console.log('signature: ', signature);
console.log('response: ');

axios({
    method: 'post',
    url: 'https://api-ssn.prakerja.go.id' + endpoint,
    headers: {
        'Content-Type': 'application/json',
        'client_code': client_code,
        'signature': signature,
        'timestamp': timestamp
    },
    data: {
        'redeem_code': redeem_code
    }
})
.then((response) => {
    console.log(response.data)
})
.catch((error) => {
    console.log(error.response.data)
});