use crate::models::Post;
use anyhow::Result;
use csv::Writer;
use std::fs::File;

pub fn save_to_csv(posts: &[Post], filename: &str) -> Result<()> {
    let file = File::create(filename)?;
    let mut writer = Writer::from_writer(file);

    for post in posts {
        writer.serialize(post)?;
    }

    writer.flush()?;
    println!("Saved {} posts to {}", posts.len(), filename);
    Ok(())
}
