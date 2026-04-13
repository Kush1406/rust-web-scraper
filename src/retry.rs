use anyhow::Result;

pub async fn fetch_with_retry(url: &str, max_retries: usize) -> Result<String> {
    let mut attempts = 0;
    loop {
        attempts += 1;

        // Try to fetch
        match crate::scraper::fetch_page(url).await {
            // Success!
            Ok(html) => return Ok(html),
            Err(e) => {
                if attempts >= max_retries {
                    // Give up
                    return Err(e);
                }

                // Calculate wait time
                let wait_secs = 2_u64.pow((attempts - 1) as u32);

                eprintln!(
                    "Retry {}/{} failed for {}: {}, Retrying in {} seconds",
                    attempts, max_retries, url, e, wait_secs
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(wait_secs)).await;
            }
        }
    }
}
