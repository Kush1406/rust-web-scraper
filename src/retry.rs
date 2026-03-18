use anyhow::Result;
use std::time::Duration;

pub async fn fetch_with_retry(url: &str, max_retries: usize) -> Result<String> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        match crate::scraper::fetch_page(url).await {
            Ok(html) => return Ok(html),
            Err(e) => {
                if attempts < max_retries {
                    eprintln!("Retry {}/{} for {}: {}", attempts, max_retries, url, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
            Err(e) => return Err(e),
        }
    }
}
