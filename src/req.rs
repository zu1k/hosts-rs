use std::time::Duration;
use reqwest::blocking::Client;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

pub const GITHUB_520: &str = "https://raw.githubusercontent.com/521xueweihan/GitHub520/main/hosts";

pub fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent(USER_AGENT)
        .build()?;

    let body = client.get(url).send()?
        .text()?;

    
    Ok(body)
}