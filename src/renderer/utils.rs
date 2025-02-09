pub fn date_and_time(modification_date: &Option<chrono::NaiveDateTime>) -> String {
    match modification_date {
        Some(modification_date) => {
            // let local_time = chrono::Local.from_local_datetime(modification_date).unwrap();
            // local_time.format("%d %b %Y").to_string().to_lowercase()
            todo!()
        }
        None => String::new(),
    }
}

pub fn tag_links_to_timeline(tags: Option<Vec<String>>) -> String {
    match tags {
        Some(tags) => {
            let mut result = String::new();
            for tag in tags {
                result.push_str(&format!(
                    r#"<a href="timeline.html?filter=tag::{}" class="tagbtn btn btn-primary">{}</a>"#,
                    tag, tag
                ));
            }
            result
        }
        None => String::new(),
    }
}
