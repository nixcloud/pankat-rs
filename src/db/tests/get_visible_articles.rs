#[cfg(test)]
mod tests {
    use crate::db::article::{get_visible_articles, set};
    use crate::db::tests::establish_connection_and_initialize_schema;
    use diesel::sqlite::SqliteConnection;

    use crate::articles::ArticleWithTags;

    #[test]
    fn test_db_get_visible_articles() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: None,
            summary: Some("Test".to_string()),
            tags: Some(vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string(),
            ]),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        set(&mut conn, &article_with_tags1).unwrap();

        let article_with_tags2 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set2.mdwn".to_string(),
            dst_file_name: "test_db_set2.html".to_string(),
            title: Some("Test2".to_string()),
            modification_date: None,
            summary: Some("Test2".to_string()),
            tags: Some(vec!["test2".to_string(), "test3".to_string()]),
            series: Some("Test2".to_string()),
            draft: None,
            special_page: Some(true),
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        set(&mut conn, &article_with_tags2).unwrap();

        let article_with_tags3 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set3.mdwn".to_string(),
            dst_file_name: "test_db_set3.html".to_string(),
            title: Some("Test2".to_string()),
            modification_date: None,
            summary: Some("Test2".to_string()),
            tags: Some(vec!["test2".to_string(), "test3".to_string()]),
            series: Some("Test2".to_string()),
            draft: Some(true),
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        set(&mut conn, &article_with_tags3).unwrap();

        match get_visible_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 1);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
