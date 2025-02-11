use crate::articles::ArticleWithTags;
use std::error::Error;

pub fn title(input: &str, article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let title = input.trim().to_string();
        article.title = Some(title);
        Ok("".to_string())
    }
}
