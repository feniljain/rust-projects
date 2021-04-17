#![feature(hash_set_entry)]
use csv;
use serde_derive::Deserialize;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Deserialize)]
struct RawPost {
    title: String,
    tags: HashSet<String>,
}

struct Post {
    title: String,
    tags: HashSet<Rc<String>>,
}

fn main() -> Result<(), std::io::Error> {
    let mut all_tags: HashSet<Rc<String>> = HashSet::new();
    let mut posts: Vec<Post> = Vec::new();

    let dir = std::fs::read_dir("posts")?;
    for entry in dir {
        let entry = entry?;

        let contents = std::fs::read_to_string(entry.path())?;
        let post: RawPost = toml::from_str(&contents)?;

        let raw_post: RawPost = toml::from_str(&contents)?;
        let mut post_tags: HashSet<Rc<String>> = HashSet::new();

        for tag in raw_post.tags {
            let tag = Rc::new(tag);
            let tag = all_tags.get_or_insert(tag);
            post_tags.insert(tag.clone());
        }

        let post = Post {
            title: raw_post.title,
            tags: post_tags,
        };

        posts.push(post);
    }

    gen_csv(&all_tags, &posts)?;

    Ok(())
}

fn gen_csv(all_tags: &HashSet<Rc<String>>, posts: &[Post]) -> Result<(), std::io::Error> {
    let mut writer = csv::Writer::from_path("tag_matrix.csv")?;

    let mut header = vec!["Title"];
    for tag in all_tags.iter() {
        header.push(tag);
    }

    writer.write_record(header)?;

    for post in posts {
        let mut record = vec![post.title.as_str()];
        for tag in all_tags {
            let field = if post.tags.contains(tag) {
                "true"
            } else {
                "false"
            };
            record.push(field);
        }
        writer.write_record(record)?;
    }

    writer.flush()?;
    Ok(())
}
