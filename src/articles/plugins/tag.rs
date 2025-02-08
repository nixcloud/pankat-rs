use crate::articles::NewArticle;
use std::error::Error;

pub fn tag(input: &str, article: &mut NewArticle) -> Result<String, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let tag = input.to_string();
        article.tags = Some(tag.split_whitespace().map(|s| s.to_string()).collect());
        Ok("".to_string())
    }
}
