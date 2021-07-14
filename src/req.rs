use std::time::Duration;
use reqwest::blocking::Client;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

pub const GITHUB_520: [&str;3] = [
    "https://api.hellogithub.com/GitHub520/hosts",
    "https://raw.hellogithub.com/hosts", 
    "https://raw.githubusercontent.com/521xueweihan/GitHub520/main/hosts"
];

pub fn fetch() -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(USER_AGENT)
        .build().unwrap();

    for url in GITHUB_520 {
        if let Ok(resp) = client.get(url).send() {
            if let Ok(body) = resp.text() {
                println!("valid url: {}", url);
                return Some(body)
            }
        }
    }
    
    None
}