const crypto = require('crypto-js');

// if using this value bellow should output 943c20a8e36b6243f576404b00fffdc86411ce1f
const text = 'myclient_code1516239022POST/api/v1/integration/payment/redeem-code/commit{"redeem_code":"code"}';
const key = 'my_sign_key';

const output = crypto.HmacSHA1(text, key).toString(crypto.enc.Hex);
console.log(output);