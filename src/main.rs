mod models;
mod parser;
mod scraper;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let html = scraper::fetch_page("https://news.ycombinator.com/").await?;

    // println!("Fetched {} bytes", html.len());
    // println!("First 500 chars:\n{}\n", &html[..500.min(html.len())]);

    parser::debug_parse(&html);

    let posts = parser::parse_posts(&html)?;

    println!("Found {} posts\n", posts.len());

    for (i, post) in posts.iter().take(5).enumerate() {
        println!("{}. {}", i + 1, post.title);
        println!("   URL: {:?}", post.url);
        println!("   Points: {:?}", post.points);
        println!("   Comments: {:?}", post.comments);
        println!("   Author: {:?}", post.author);
        println!("   Time: {:?}", post.time);
        println!();
    }
    Ok(())
}
