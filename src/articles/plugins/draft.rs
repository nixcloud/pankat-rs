use crate::articles::Article;
use std::error::Error;

pub fn draft(_input: &str, article: &mut Article) -> Result<String, Box<dyn Error>> {
    article.draft = Some(true);
    Ok("".to_string())
}
