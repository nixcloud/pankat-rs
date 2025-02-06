use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn title(input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let title = input.trim().to_string();
        article.title = Some(title);
        Ok(PluginResult {
            name: "title".to_string(),
            output: "".to_string(),
        })
    }
}
