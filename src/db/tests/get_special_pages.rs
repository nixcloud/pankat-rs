#[cfg(test)]
mod tests {
    use crate::db::article::{
        get_prev_and_next_article, get_special_pages, get_visible_articles, set,
    };
    use crate::db::tests::establish_connection_and_initialize_schema;
    use diesel::sqlite::SqliteConnection;

    use crate::articles::ArticleWithTags;
    use chrono::NaiveDateTime;

    #[test]
    fn test_db_get_special_pages() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();
        let parsed_time1 =
            NaiveDateTime::parse_from_str("2001-01-01 01:01", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: Some(parsed_time1),
            summary: Some("Test".to_string()),
            tags: Some(vec!["test1 test2 test3".to_string()]),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags1);
        assert!(res.is_ok());

        match get_visible_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 1);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let parsed_time2 =
            NaiveDateTime::parse_from_str("2003-01-01 01:01", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags2 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set2.mdwn".to_string(),
            dst_file_name: "test_db_set2.html".to_string(),
            title: Some("Test2".to_string()),
            modification_date: Some(parsed_time2),
            summary: Some("Test2".to_string()),
            tags: Some(vec!["test2".to_string(), "test3".to_string()]),
            series: Some("Test2".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags2);
        assert!(res.is_ok());

        match get_visible_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 2);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        let parsed_time3 =
            NaiveDateTime::parse_from_str("2002-01-01 01:01", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags3 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set3.mdwn".to_string(),
            dst_file_name: "test_db_set3.html".to_string(),
            title: Some("Test3".to_string()),
            modification_date: Some(parsed_time3),
            summary: Some("Test3".to_string()),
            tags: Some(vec!["test3".to_string()]),
            series: Some("Test3".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags3);
        assert!(res.is_ok());

        let parsed_time_draft =
            NaiveDateTime::parse_from_str("2002-02-02 02:02", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags_draft = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set4.mdwn".to_string(),
            dst_file_name: "test_db_set4.html".to_string(),
            title: Some("Test4".to_string()),
            modification_date: Some(parsed_time_draft),
            summary: Some("Test4".to_string()),
            tags: Some(vec!["test4".to_string()]),
            series: Some("Test4".to_string()),
            draft: Some(true),
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags_draft);
        assert!(res.is_ok());

        let parsed_time_special_page1 =
            NaiveDateTime::parse_from_str("2002-02-02 02:02", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags_special_page1 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set5.mdwn".to_string(),
            dst_file_name: "test_db_set5.html".to_string(),
            title: Some("Test5".to_string()),
            modification_date: Some(parsed_time_special_page1),
            summary: Some("Test5".to_string()),
            tags: Some(vec!["test5".to_string()]),
            series: Some("Test5".to_string()),
            draft: Some(true),
            special_page: Some(true),
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags_special_page1);
        assert!(res.is_ok());

        let parsed_time_special_page2 =
            NaiveDateTime::parse_from_str("2002-03-03 03:03", "%Y-%m-%d %H:%M").unwrap();
        let article_with_tags_special_page2 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set9.mdwn".to_string(),
            dst_file_name: "test_db_set9.html".to_string(),
            title: Some("Test9".to_string()),
            modification_date: Some(parsed_time_special_page2),
            summary: Some("Test9".to_string()),
            tags: Some(vec!["test9".to_string()]),
            series: Some("Test9".to_string()),
            draft: Some(true),
            special_page: Some(true),
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags_special_page2);
        assert!(res.is_ok());

        match get_special_pages(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 2);
                assert_eq!(articles_with_tags[0].id.unwrap(), 5);
                assert_eq!(articles_with_tags[1].id.unwrap(), 6);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        let res = get_prev_and_next_article(&mut conn, 5);
        match res {
            Ok(article_neighbours) => {
                assert_eq!(article_neighbours.prev, None);
                assert_eq!(article_neighbours.next, None);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        let res = get_prev_and_next_article(&mut conn, 6);
        match res {
            Ok(article_neighbours) => {
                assert_eq!(article_neighbours.prev, None);
                assert_eq!(article_neighbours.next, None);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
