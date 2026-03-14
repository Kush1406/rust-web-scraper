mod models;
mod parser;
mod scraper;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let html = scraper::fetch_page("https://news.ycombinator.com").await?;
    // DEBUG: Check if we got HTML
    println!("Fetched {} bytes", html.len());
    println!("First 500 chars:\n{}\n", &html[..500.min(html.len())]);

    let posts = parser::parse_posts(&html)?;
    println!("Found {} posts", posts.len());

    Ok(())
}
