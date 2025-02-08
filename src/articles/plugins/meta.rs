use crate::articles::NewArticle;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use regex::Regex;
use std::error::Error;
use std::time::SystemTime;

pub fn meta(input: &str, article: &mut NewArticle) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}").unwrap();
    if let Some(mat) = re.find(&input) {
        if let Ok(parsed_time) = NaiveDateTime::parse_from_str(mat.as_str(), "%Y-%m-%d %H:%M") {
            let system_time: SystemTime = Utc.from_utc_datetime(&parsed_time).into();
            article.modification_date = Some(system_time);
            return Ok("".to_string())
        } else {
            Err("Argument contains invalid characters (newlines or tabs)".into())
        }
    } else {
        Err("Argument contains invalid characters (newlines or tabs)".into())
    }
}
