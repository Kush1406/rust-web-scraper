use std::io::SeekFrom;

use crate::models::Post;
use anyhow::Result;
use scraper::{Html, Selector};

pub fn parse_posts(html: &str) -> Result<Vec<Post>> {
    let document = Html::parse_document(html);
    let post_selector = Selector::parse("tr.athing").unwrap();

    let title_selector = Selector::parse("tr.athing > span.titleline > a").unwrap();
    let url_selector = Selector::parse("tr.athing > span.titleline > a[href]").unwrap();
    let points_selector = Selector::parse("tr.athing + tr > td.subtext > span.subline > span.score").unwrap();
    let comments_selector = Selector::parse("tr.athing + tr > td.subtext > a").unwrap();
    let author_selector = Selector::parse("tr.athing + tr > td").unwrap();
}
