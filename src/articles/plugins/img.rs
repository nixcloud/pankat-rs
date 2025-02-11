use crate::articles::ArticleWithTags;
use std::error::Error;

pub fn img(input: &str, _article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    let mut parts = input.split_whitespace();
    let img_url = parts.next().unwrap_or("").to_string();
    let attributes = parts.collect::<Vec<&str>>().join(" ");

    let out = format!(
        r#"<a href="{}"><img src="{}" {}></a>"#,
        img_url, img_url, attributes
    );
    Ok(out.to_string())
}
