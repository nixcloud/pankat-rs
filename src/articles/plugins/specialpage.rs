use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn specialpage(_input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    article.special_page = Some(true);
    Ok(PluginResult {
        name: "specialpage".to_string(),
        output: "".to_string(),
    })
}
