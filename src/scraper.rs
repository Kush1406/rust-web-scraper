use anyhow::Result;

pub async fn fetch_page(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

async fn fetch_with_retry(url: &str, max_retries: usize) -> Result<String> {
    let mut attempts = 0;
    loop {
        match fetch_page(url).await {
            Ok(html) => return Ok(html),
            Err(e) => {
                if attempts < max_retries {
                    attempts += 1;
                    eprintln!("Retry {}/{} for {}: {}", attempts, max_retries, url, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
            Err(e) => return Err(e),
        }
    }
}
