use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn img(input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    if input.contains('\n') || input.contains('\t') {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    } else {
        let mut parts = input.split_whitespace();
        let img = parts.next().unwrap_or("").to_string();
        let subarg = parts.collect::<Vec<&str>>().join(" ");
        Ok(PluginResult {
            name: "img".to_string(),
            output: format!("<a href=\"{}\"><img src=\"{}\"></a>", img, subarg),
        })
    }
}
