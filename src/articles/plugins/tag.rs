use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn tag(input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let tag = input.to_string();
        article.tags = Some(tag.split_whitespace().map(|s| s.to_string()).collect());
        Ok(PluginResult {
            name: "tag".to_string(),
            output: "".to_string(),
        })
    }
}
