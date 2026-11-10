use include_dir::{Dir, include_dir};

use crate::markdown::render_markdown;

static POSTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/posts");

#[derive(Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub html: String,
}

pub fn load_posts() -> Vec<Post> {
    let mut posts = Vec::new();

    for file in POSTS.files() {
        let Some(name) = file.path().file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !name.ends_with(".md") {
            continue;
        }

        let Some(markdown) = file.contents_utf8() else {
            continue;
        };

        let slug = name.trim_end_matches(".md").to_string();

        let title = markdown
            .lines()
            .find_map(|line| line.strip_prefix("# "))
            .map(str::trim)
            .unwrap_or(&slug)
            .to_string();

        posts.push(Post {
            slug,
            title,
            html: render_markdown(markdown),
        });
    }

    posts.sort_by(|a, b| b.slug.cmp(&a.slug));
    posts
}
