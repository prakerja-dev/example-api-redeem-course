use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub struct Message {
    client_code: String,
    timestamp: String,
    method: String,
    endpoint: String,
    body: String,
}

impl Message {
    pub fn new(
        client_code: String,
        timestamp: String,
        method: String,
        endpoint: String,
        body: String,
    ) -> Result<Self, String> {
        if client_code.is_empty()
            || timestamp.is_empty()
            || method.is_empty()
            || endpoint.is_empty()
            || body.is_empty()
        {
            return Err("Please make sure your fields are not empty".to_owned());
        }

        Ok(Message {
            client_code,
            timestamp,
            method,
            endpoint,
            body,
        })
    }

    pub fn concate(&self) -> String {
        format!(
            "{}{}{}{}{}",
            &self.client_code, &self.timestamp, &self.method, &self.endpoint, &self.body
        )
    }
}

pub struct Signature {
    secret_key: String,
    message: String,
}

impl Signature {
    pub fn new(secret_key: String, message: String) -> Result<Self, String> {
        if secret_key.is_empty() || message.is_empty() {
            return Err("Please make sure your fields are not empty".to_owned());
        }

        Ok(Signature {
            secret_key,
            message,
        })
    }

    pub fn generate(&self) -> String {
        let mut mac =
            HmacSha1::new_from_slice(self.secret_key.as_bytes()).expect("Secret key is not found");
        mac.update(self.message.as_bytes());
        format!("{:x}", mac.finalize().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_concate() {
        let client_code = "myclient_code".to_owned();
        let method = "POST".to_owned();
        let timestamp = "1516239022".to_owned();
        let endpoint = "/api/v1/integration/payment/redeem-code/commit".to_owned();
        let body = "{\"redeem_code\":\"code\"}".to_owned();

        let msg = Message::new(client_code, timestamp, method, endpoint, body).unwrap();
        let result = msg.concate();

        let expected = "myclient_code1516239022POST/api/v1/integration/payment/redeem-code/commit{\"redeem_code\":\"code\"}".to_owned();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate() {
        let secret_key = "my_sign_key".to_owned();
        let message = "myclient_code1516239022POST/api/v1/integration/payment/redeem-code/commit{\"redeem_code\":\"code\"}".to_owned();

        let sign = Signature::new(secret_key, message).unwrap();
        let result = sign.generate();

        let expected = "943c20a8e36b6243f576404b00fffdc86411ce1f";

        assert_eq!(result, expected);
    }
}
