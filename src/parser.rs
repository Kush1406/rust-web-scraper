use core::time;
use std::io::SeekFrom;

use crate::models::Post;
use anyhow::Result;
use scraper::{Html, Selector};

pub fn parse_posts(html: &str) -> Result<Vec<Post>> {
    let document = Html::parse_document(html);
    let post_selector = Selector::parse("tr.athing").unwrap();

    let title_selector = Selector::parse("tr.athing > span.titleline > a").unwrap();
    let url_selector = Selector::parse("tr.athing > span.titleline > a[href]").unwrap();
    let points_selector =
        Selector::parse("tr.athing + tr > td.subtext > span.subline > span.score").unwrap();
    let comments_selector = Selector::parse("tr.athing + tr > td.subtext > a").unwrap();
    let author_selector =
        Selector::parse("tr.athing + tr > td.subtext > span.subline > a").unwrap();
    let time_selector =
        Selector::parse("tr.athing + tr > td.subtext > span.subline > span.age").unwrap();

    let mut posts = Vec::new();
    for post_element in document.select(&post_selector) {
        let title = post_element
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        let url = post_element
            .select(&url_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or_default()
            .to_string();

        let points = post_element
            .select(&points_selector)
            .next()
            .map(|e| {
                e.inner_html()
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        let comments = post_element
            .select(&comments_selector)
            .next()
            .map(|e| {
                e.inner_html()
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        let author = post_element
            .select(&author_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        let time = post_element
            .select(&time_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        let post = Post {
            title,
            url: Some(url),
            points: Some(points),
            comments: Some(comments),
            author: Some(author),
            time: Some(time),
        };

        posts.push(post);
    }

    return Ok(posts);
}
