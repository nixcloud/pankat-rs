use crate::articles::ArticleWithTags;
use std::error::Error;

pub fn draft(_input: &str, article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    article.draft = Some(true);
    Ok("".to_string())
}
