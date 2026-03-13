use crate::models::Post;
use anyhow::Result;
use scraper::{Html, Selector};

pub fn parse_posts(html: &str) -> Result<Vec<Post>> {
    let document = Html::parse_document(html);

    let post_selector = Selector::parse("tr").unwrap();
    let title_selector = Selector::parse("span.titleline > a").unwrap();
    let subtext_selector = Selector::parse("td.subtext").unwrap();
    let points_selector = Selector::parse("span.score").unwrap();
    let author_selector = Selector::parse("a.hnuser").unwrap();
    let time_selector = Selector::parse("span.age a").unwrap();
    let url_selector = Selector::parse("a").unwrap();

    let all_rows: Vec<_> = document.select(&post_selector).collect();
    let mut posts = Vec::new();

    for (i, row) in all_rows.iter().enumerate() {
        if row.value().attr("class").unwrap_or("") != "athing" {
            continue;
        }

        let title = row
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let url = row
            .select(&url_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|s| s.to_string());

        let metadata_row = all_rows.get(i + 1);

        let mut points = None;
        let mut comments = None;
        let mut author = None;
        let mut time = None;

        if let Some(meta_row) = metadata_row {
            if let Some(subtext) = meta_row.select(&subtext_selector).next() {
                points = subtext.select(&points_selector).next().and_then(|e| {
                    let text = e.text().collect::<String>();
                    text.split_whitespace()
                        .next()
                        .and_then(|s| s.parse::<u32>().ok())
                });

                author = subtext
                    .select(&author_selector)
                    .next()
                    .map(|e| e.text().collect::<String>());

                time = subtext
                    .select(&time_selector)
                    .next()
                    .map(|e| e.text().collect::<String>());

                for link in subtext.select(&url_selector) {
                    let text = link.text().collect::<String>();

                    if text.contains("comment") || text == "discuss" {
                        comments = if text == "discuss" {
                            Some(0)
                        } else {
                            text.split_whitespace()
                                .next()
                                .and_then(|s| s.parse::<u32>().ok())
                        };
                        break;
                    }
                }
            };
        }

        posts.push(Post {
            title,
            url,
            points,
            comments,
            author,
            time,
        });
    }

    Ok(posts)
}
