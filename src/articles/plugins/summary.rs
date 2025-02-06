use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn summary(input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
  if input.contains('\n') || input.contains('\t') {
    Err("Argument contains invalid characters (newlines or tabs)".into())
  } else {
    let summary = input.trim().to_string();
    article.summary = Some(summary);
    Ok(PluginResult {
        name: "summary".to_string(),
        output: "".to_string(),
    })
  }
}