#[cfg(test)]
mod tests {
    use crate::articles::ArticleWithTags;
    use crate::db::article::{del_by_id, get_visible_articles, set};
    use crate::db::tests::establish_connection_and_initialize_schema;
    use diesel::sqlite::SqliteConnection;
    use std::collections::HashSet;

    #[test]
    fn test_db_del_by_id() {
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
            special_page: None,
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

        let ret = del_by_id(&mut conn, 1);
        assert!(ret.is_ok());

        match ret {
            Ok(db_reply) => {
                let assumed_result: HashSet<i32> = vec![2].into_iter().collect();
                assert_eq!(db_reply.affected_articles, assumed_result);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };

        match get_visible_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 1);
                assert_eq!(articles_with_tags[0].id, Some(2));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_db_del_by_id_series() {
        use chrono::NaiveDateTime;

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
            series: Some("Test2".to_string()),
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
            special_page: None,
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
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        set(&mut conn, &article_with_tags3).unwrap();

        let parsed_time =
            NaiveDateTime::parse_from_str("2024-07-19 14:33", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags4 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set4.mdwn".to_string(),
            dst_file_name: "test_db_set4.html".to_string(),
            title: Some("Test2".to_string()),
            modification_date: Some(parsed_time),
            summary: Some("Test2".to_string()),
            tags: Some(vec!["test2".to_string(), "test3".to_string()]),
            series: Some("Test2".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        set(&mut conn, &article_with_tags4).unwrap();

        let ret = del_by_id(&mut conn, 2);
        assert!(ret.is_ok());

        match ret {
            Ok(db_reply) => {
                let assumed_result: HashSet<i32> = vec![1, 3].into_iter().collect();
                assert_eq!(db_reply.affected_articles, assumed_result);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };

        match get_visible_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 3);
                assert_eq!(articles_with_tags[0].id, Some(4));
                assert_eq!(articles_with_tags[1].id, Some(1));
                assert_eq!(articles_with_tags[2].id, Some(3));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
