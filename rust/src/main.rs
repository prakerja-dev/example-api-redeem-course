use chrono::Utc;
use redeem_code::{Message, Signature};
use reqwest::blocking::Client;
use std::{collections::HashMap, env};

fn main() {
    // See:
    // https://prakerja.atlassian.net/wiki/spaces/AP/pages/483524940/Prerequisite
    // for list of host/domain
    //
    // at this example we use sandbox as host
    let host = "https://api-ssn.prakerja.go.id/";

    // Get client code and secret key from environment variable, keep it secure!
    let client_code = match env::var("CLIENT_CODE") {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let secret_key = match env::var("SECRET_KEY") {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let redeem_code = match env::var("REDEEM_CODE") {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let timestamp = Utc::now().timestamp().to_string();
    let method = "POST";

    // See:
    // https://prakerja.atlassian.net/wiki/spaces/AP/pages/483688640/API+Commit#Request for commit
    // or https://prakerja.atlassian.net/wiki/spaces/AP/pages/483754269/API+Check+Status#Request
    // for get status
    //
    // at this example we use commit endpoint
    let endpoint = "/api/v1/integration/payment/redeem-code/status";

    let mut body = HashMap::new();
    body.insert("redeem_code", &redeem_code);

    let msg_body = serde_json::to_string(&body).expect("Failed to serialize body");
    let m = Message::new(
        client_code.to_owned(),
        timestamp.to_owned(),
        method.to_owned(),
        endpoint.to_owned(),
        msg_body,
    );
    let message = match m {
        Ok(m) => m.concate(),
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let s = Signature::new(secret_key, message);
    let sign = match s {
        Ok(s) => s.generate(),
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let client = Client::new();
    let response = client
        .post(host)
        .header("Content-Type", "application/json")
        .header("client_code", &client_code)
        .header("signature", &sign)
        .header("timestamp", &timestamp)
        .json(&body)
        .send();

    println!("{response:?}");
}
