use crate::articles::NewArticle;
use std::error::Error;

pub fn draft(_input: &str, article: &mut NewArticle) -> Result<String, Box<dyn Error>> {
    article.draft = Some(true);
    Ok("".to_string())
}
