use crate::articles::Article;
use crate::articles::PluginResult;
use std::error::Error;

pub fn img(input: &str, _article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    let mut parts = input.split_whitespace();
    let img_url = parts.next().unwrap_or("").to_string();
    let attributes = parts.collect::<Vec<&str>>().join(" ");

    let out = format!(
        r#"<a href="{}"><img src="{}" {}></a>"#,
        img_url, img_url, attributes
    );
    //println!("xxxxxxxxxxxxxxxxxx {}", input);
    Ok(PluginResult {
        name: "img".to_string(),
        output: out.to_string(),
    })
}
