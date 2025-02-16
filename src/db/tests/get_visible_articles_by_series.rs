#[cfg(test)]
mod tests {
    use crate::db::article::{get_visible_articles_by_series, set};
    use crate::db::tests::establish_connection_and_initialize_schema;
    use diesel::sqlite::SqliteConnection;

    use crate::articles::ArticleWithTags;

    #[test]
    fn test_db_get_visible_articles_by_series_some() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: None,
            summary: Some("Test".to_string()),
            tags: Some(vec!["test1 test2 test3".to_string()]),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let _ = set(&mut conn, &article_with_tags1).unwrap();

        match get_visible_articles_by_series(&mut conn, "Test") {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 1);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_db_get_visible_articles_by_series_none() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: None,
            summary: Some("Test".to_string()),
            tags: Some(vec!["test1 test2 test3".to_string()]),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let _ = set(&mut conn, &article_with_tags1).unwrap();

        match get_visible_articles_by_series(&mut conn, "asdfasdfasdf") {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 0);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
