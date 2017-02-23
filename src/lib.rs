extern crate hyper;
#[macro_use]
extern crate serde_json;
extern crate env_logger;

use hyper::Client;
use std::io::Read;

#[derive(Debug)]
pub enum GolosdError {
    CallFailed,
}

pub fn call(api: String,
            api_method: String,
            args: Vec<String>)
            -> Result<serde_json::Value, GolosdError> {
    const RPC_ENDPOINT: &'static str = "http://node.golos.ws/rpc";

    let value = json!({
        "jsonrpc": "2.0",
        "method": "call",
        "id": "1",
        "params": [api, api_method, args]
    });

    let client = Client::new();

    let mut res = client.post(RPC_ENDPOINT)
        .body(&serde_json::to_string(&value).unwrap())
        .send()
        .unwrap();

    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    let json: serde_json::Value = serde_json::from_str(&s).unwrap();

    match json["error"].is_string() {
        false => Ok(json),
        true => Err(GolosdError::CallFailed),
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    #[test]
    fn get_dynamic_props_rpc_call_succeeds() {
        let api = "database_api".to_string();
        let api_method = "get_dynamic_global_properties".to_string();
        let args = vec![];
        let response_map = json!(call(api, api_method, args).unwrap());
        assert!(response_map["result"]["head_block_number"].as_u64().unwrap() > 3000000);
    }

    #[test]
    fn get_content_rpc_call_succeeds() {
        let api = "database_api".to_string();
        let api_method = "get_content".to_string();
        let args = vec!["hipster".to_string(), "iniciativa-kiber-fonda-po-podderzhke-otkrytogo-iskhodnogo-koda-v-golose".to_string()];
        let response_map = json!(call(api, api_method, args).unwrap());
        assert!(response_map["result"]["title"].as_str().unwrap() == "Инициатива кибер•Фонда по поддержке открытого исходного кода в Голосе");
    }
}