mod models;
mod parser;
mod scraper;
mod writer;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Fetching Hacker News front page...");
    let html = scraper::fetch_page("https://news.ycombinator.com").await?;
    // DEBUG: Check if we got HTML
    // println!("Fetched {} bytes", html.len());
    // println!("First 500 chars:\n{}\n", &html[..500.min(html.len())]);

    println!("Parsing posts...");
    let posts = parser::parse_posts(&html)?;

    println!("Found {} posts", posts.len());

    for (i, post) in posts.iter().take(5).enumerate() {
        println!("{}. {}", i + 1, post.title);
        println!("URL: {:?}", post.url);
        println!("Points: {:?}", post.points);
        println!("Comments: {:?}", post.comments);
        println!("Author: {:?}", post.author);
        println!("Time: {:?}", post.time);
        println!("---");
    }

    // Save to CSV
    writer::save_to_csv(&posts, "hacker_news_posts.csv")?;

    Ok(())
}
