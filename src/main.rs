mod models;
mod parser;
mod scraper;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let html = scraper::fetch_page("https://news.ycombinator.com").await?;
    println!("Fetched {} bytes", html.len());
    Ok(())
}
