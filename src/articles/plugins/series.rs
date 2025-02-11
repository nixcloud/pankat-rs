use crate::articles::ArticleWithTags;
use std::error::Error;

pub fn series(input: &str, article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let series = input.to_string();
        article.series = Some(series);
        Ok("".to_string())
    }
}
