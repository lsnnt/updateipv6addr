use std::env;
use serde_json::json;
use reqwest::header::{HeaderMap};
use reqwest::Client;
use dotenv::dotenv;
async fn getip() -> String {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        "curl/8.7.1".parse().unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let response = client.get("https://ip.se/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .trim()
        .replace("\n", "");
    return response;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let ip = getip().await;
    let api_key = env::var("CFTOKEN").unwrap();
    let zone_id = env::var("ZONE_ID").unwrap();
    let dns_record_id = env::var("DNS_RECORD_ID").unwrap();
    let domain  = env::var("DOMAIN").unwrap();

    let mut header = HeaderMap::new();
    header.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    header.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    let data = json!(
        {
          "name": domain,
          "ttl": 3600,
          "type": "AAAA",
          "comment": format!("Api called from {}",ip),
          "content": ip,
          "proxied": false,
        }
    );
    let client = Client::builder().default_headers(header).build().unwrap();
    let response = client
        .put(format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",zone_id, dns_record_id))
        .json(&data)
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}