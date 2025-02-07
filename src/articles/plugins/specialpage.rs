use crate::articles::Article;
use std::error::Error;

pub fn specialpage(_input: &str, article: &mut Article) -> Result<String, Box<dyn Error>> {
    article.special_page = Some(true);
    Ok("".to_string())
}
