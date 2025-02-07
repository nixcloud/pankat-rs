use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn draft(_input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    article.draft = Some(true);
    Ok(PluginResult {
        name: "draft".to_string(),
        output: "".to_string(),
    })
}
