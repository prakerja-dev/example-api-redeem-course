using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Security.Cryptography;
using System.IO;
using System.Net;
using System.Net.Http.Headers;

DateTime now = DateTime.Now;

string client_code = "";
string redeem_code = "";
string key = "";
string endpoint = "/api/v1/integration/payment/redeem-code/status";
long timestamp = ((DateTimeOffset)now).ToUnixTimeSeconds();
string text = client_code + timestamp + "POST" + endpoint + "{\"redeem_code\":\"" + redeem_code + "\"}";

byte[] bytkey = Encoding.ASCII.GetBytes(key);
HMACSHA1 myhmacsha1 = new HMACSHA1(bytkey);
byte[] byteArray = Encoding.ASCII.GetBytes(text);
MemoryStream stream = new MemoryStream(byteArray);
string signature = myhmacsha1.ComputeHash(stream).Aggregate("", (s, e) => s + String.Format("{0:x2}", e), s => s);

Console.WriteLine("timestamp: " + timestamp.ToString());
Console.WriteLine("signature: " + signature);

HttpClient httpClient = new HttpClient();
var request = new HttpRequestMessage(new HttpMethod("POST"), "https://api-ssn.prakerja.go.id" + endpoint);

request.Content = new StringContent("{\"redeem_code\":\"" + redeem_code + "\"}");
request.Content.Headers.ContentType = MediaTypeHeaderValue.Parse("application/json");

request.Headers.TryAddWithoutValidation("client_code", client_code);
request.Headers.TryAddWithoutValidation("signature", signature);
request.Headers.TryAddWithoutValidation("timestamp", timestamp.ToString());

HttpResponseMessage response = await httpClient.SendAsync(request);
HttpContent content = response.Content;
var result = content.ReadAsStringAsync().GetAwaiter().GetResult();

Console.WriteLine("response: " + result);