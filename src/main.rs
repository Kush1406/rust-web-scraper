mod models;
mod parser;
mod rate_limiter;
mod scraper;
mod writer;

use anyhow::Result;
use rate_limiter::RateLimiter;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    // Generate URLs for the first five pages
    let urls: Vec<String> = (1..=5)
        .map(|page| {
            if page == 1 {
                "https://news.ycombinator.com".to_string()
            } else {
                format!("https://news.ycombinator.com/news?p={}", page)
            }
        })
        .collect();

    println!(
        "Will scrape {} pages with rate limiting (max 2 concurrent):\n",
        urls.len()
    );

    // Create rate limiter - max 2 concurrent requests
    let rate_limiter = RateLimiter::new(2);
    let start = Instant::now();

    // spawn tasks for each URL
    let mut tasks = vec![];

    for (i, url) in urls.into_iter().enumerate() {
        let url = url.clone();
        let limiter = rate_limiter.clone();

        let task = tokio::spawn(async move {
            println!("[Page {}] Waiting for the rate limiter...", i + 1);

            // Use rate limiter to control concurrency
            let result = limiter
                .execute(|| async {
                    println!("[Page {}] Starting to fetch: {}", i + 1, url);
                    let html = scraper::fetch_page(&url).await?;
                    let posts = parser::parse_posts(&html)?;
                    println!("[Page {}] Completed - found {} posts", i + 1, posts.len());
                    Ok::<Vec<models::Post>, anyhow::Error>(posts)
                })
                .await;

            result
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    println!("\nWaiting for all pages to complete...");
    let results = futures::future::join_all(tasks).await;

    // Collect all posts
    let mut all_posts = Vec::new();
    let mut errors = 0;

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(posts)) => {
                all_posts.extend(posts);
            }
            Ok(Err(e)) => {
                eprintln!("[Page {}] Error during scraping: {}", i + 1, e);
                errors += 1;
            }
            Err(e) => {
                eprintln!("[Page {}] Task panic: {}", i + 1, e);
                errors += 1;
            }
        }
    }

    let elapsed = start.elapsed();

    // Print summary
    println!("\n=== SCRAPING COMPLETE ===");
    println!("Total posts scraped: {}", all_posts.len());
    println!("Total errors: {}", errors);
    println!("Total time taken: {:.2}", elapsed.as_secs_f64());

    // Save to CSV
    if !all_posts.is_empty() {
        writer::save_to_csv(&all_posts, "hacker_news_posts.csv")?;
    } else {
        println!("No posts to save.");
    }

    Ok(())
}
