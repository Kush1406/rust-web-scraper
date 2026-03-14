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
    let link_selector = Selector::parse("a").unwrap();

    let all_rows: Vec<_> = document.select(&post_selector).collect();
    let mut posts = Vec::new();

    for (i, row) in all_rows.iter().enumerate() {
        let class = row.value().attr("class").unwrap_or("");
        let has_athing = class.split_whitespace().any(|c| c == "athing");
        if !has_athing {
            continue;
        }

        let title = row
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let url = row
            .select(&title_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|s| s.to_string());

        let metadata_row = all_rows.get(i + 1);

        // DEBUG: Print what we're looking at
        println!("\n--- Post at index {} ---", i);
        println!("Post class: {}", class);
        println!("Title: {}", title);

        if let Some(meta_row) = metadata_row {
            let meta_class = meta_row.value().attr("class").unwrap_or("(no class)");
            println!("Next row class: {}", meta_class);

            // Print the HTML of the next row
            println!(
                "Next row HTML preview: {}",
                &meta_row.html()[..200.min(meta_row.html().len())]
            );
        } else {
            println!("No next row!");
        }

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

                for link in subtext.select(&link_selector) {
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

pub fn debug_parse(html: &str) {
    let document = Html::parse_document(html);

    let selectors = vec!["tr.athing", "tr", ".athing", "table"];

    for selector_str in selectors {
        let selector = Selector::parse(selector_str).unwrap();
        let count = document.select(&selector).count();
        println!("Selector '{}' found {} elements", selector_str, count);
    }

    let tr_selector = Selector::parse("tr").unwrap();
    println!("\nFirst 5 <tr> elements:");
    for (i, tr) in document.select(&tr_selector).take(5).enumerate() {
        let class = tr.value().attr("class").unwrap_or("(no class)");
        let id = tr.value().attr("id").unwrap_or("(no id)");
        println!("  {}. class='{}' id='{}'", i + 1, class, id);
    }
}
