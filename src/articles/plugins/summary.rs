use crate::articles::Article;
use std::error::Error;

pub fn summary(input: &str, article: &mut Article) -> Result<String, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let summary = input.trim().to_string();
        article.summary = Some(summary);
        Ok("".to_string())
    }
}
