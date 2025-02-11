use crate::articles::ArticleWithTags;
use chrono::NaiveDateTime;
use regex::Regex;
use std::error::Error;

pub fn meta(input: &str, article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}").unwrap();
    if let Some(mat) = re.find(&input) {
        if let Ok(parsed_time) = NaiveDateTime::parse_from_str(mat.as_str(), "%Y-%m-%d %H:%M") {
            article.modification_date = Some(parsed_time);
            return Ok("".to_string());
        } else {
            Err("Argument contains invalid characters (newlines or tabs)".into())
        }
    } else {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    }
}
