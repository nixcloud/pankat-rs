use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn series(input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
  if input.contains('\n') || input.contains('\t') {
    Err("Argument contains invalid characters (newlines or tabs)".into())
  } else {
    let series = input.to_string();
    article.series = Some(series);
    Ok(PluginResult {
        name: "series".to_string(),
        output: "".to_string(),
    })
  }
}