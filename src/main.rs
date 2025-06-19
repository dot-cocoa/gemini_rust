use dotenvy::dotenv;
use std::env;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use std::collections::HashMap;
use serde_json::json;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = Client::new();
    let mut head = HeaderMap::new();
    let apikey = env::var("API_KEY")?;
    let mut message = String::new();

    println!("Promptを入力してください。:");
    std::io::stdin().read_line(&mut message).unwrap();
    head.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": message.trim().to_string()
                    }
                ]
            }
        ]
    });

    let requrl = format!( "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",apikey);
    let res = client.post(requrl)
        .headers(head)
        .json(&body)
        .send()?
        .text()?;
    let v: serde_json::Value = serde_json::from_str(&res)?;
    let resptext =&v["candidates"][0]["content"]["parts"][0]["text"];
    println!("{}",resptext);
    Ok(())
}
