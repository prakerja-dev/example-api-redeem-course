<?php
$client_code = '';
$redeem_code = '';
$key = '';
$endpoint = '/api/v1/integration/payment/redeem-code/status';
$timestamp = time();
$text = $client_code . $timestamp . 'POST' . $endpoint . '{"redeem_code":"' . $redeem_code . '"}';
$signature = hash_hmac('sha1', $text, $key);

print_r('timestamp: ' . $timestamp);
print_r("\n");
print_r('signature: ' . $signature);
print_r("\n");

$headers = [
    'Content-Type: application/json',
    'client_code: ' . $client_code,
    'signature: ' . $signature,
    'timestamp: ' . $timestamp,
];
$data = [
    'redeem_code' => $redeem_code
];

$ch = curl_init();
curl_setopt($ch, CURLOPT_URL,            'https://api-ssn.prakerja.go.id' . $endpoint );
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
curl_setopt($ch, CURLOPT_POST,           1);
curl_setopt($ch, CURLOPT_HTTPHEADER,     $headers); 
curl_setopt($ch, CURLOPT_POSTFIELDS,     json_encode($data));
$result = curl_exec($ch);
curl_close ($ch);

print_r("result: \n");
print_r($result);