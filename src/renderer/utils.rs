pub fn date_and_time(modification_date: &Option<chrono::NaiveDateTime>) -> String {
    match modification_date {
        Some(modification_date) => modification_date
            .format("%d %b %Y")
            .to_string()
            .to_lowercase(),
        None => String::new(),
    }
}


