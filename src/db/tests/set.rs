#[cfg(test)]
mod tests {
    use crate::db::article::{get_all_articles, get_tags_for_article, set};
    use crate::db::tests::establish_connection_and_initialize_schema;
    use diesel::sqlite::SqliteConnection;
    use std::collections::HashSet;

    use crate::articles::ArticleWithTags;

    #[test]
    fn test_db_set() {
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
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let ret = set(&mut conn, &article_with_tags1);

        match ret {
            Ok(db_reply) => {
                let assumed_result: HashSet<i32> = vec![1].into_iter().collect();
                assert_eq!(db_reply.affected_articles, assumed_result);
            }
            Err(e) => {
                panic!("Failed to set article with tags: {}", e);
            }
        }

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

        match get_all_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 2);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_db_set_update() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

        let original_tags = vec!["test1".to_string(), "test2".to_string()];
        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: None,
            summary: Some("Test".to_string()),
            tags: Some(original_tags.clone()),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };
        let res = set(&mut conn, &article_with_tags1);

        match res {
            Ok(db_reply) => {
                let assumed_result: HashSet<i32> = vec![1].into_iter().collect();
                assert_eq!(db_reply.affected_articles, assumed_result);
            }
            Err(_) => {}
        }

        let new_tags = vec!["test2".to_string(), "test3".to_string()];

        let article_with_tags2 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test1".to_string()),
            modification_date: None,
            summary: Some("Test2".to_string()),
            tags: Some(new_tags.clone()),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let res = set(&mut conn, &article_with_tags2);

        match res {
            Ok(db_reply) => {
                let assumed_result: HashSet<i32> = vec![1].into_iter().collect();
                assert_eq!(db_reply.affected_articles, assumed_result);
            }
            Err(_) => {}
        }

        match get_all_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 1);
                println!(
                    "articles_with_tags[0].tags: {:?}",
                    articles_with_tags[0].tags
                );
                println!("new_tags: {:?}", new_tags);
                assert_eq!(articles_with_tags[0].tags, Some(new_tags));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_db_set_empty_src_file_name() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

        let original_tags = vec!["test1".to_string(), "test2".to_string()];
        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: None,
            summary: Some("Test".to_string()),
            tags: Some(original_tags.clone()),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };
        let ret = set(&mut conn, &article_with_tags1);
        assert!(ret.is_err());

        match get_all_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 0);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_db_set_empty_dst_file_name() {
        let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

        let original_tags = vec!["test1".to_string(), "test2".to_string()];
        let article_with_tags1 = ArticleWithTags {
            id: None,
            src_file_name: "test_db_set1.mdwn".to_string(),
            dst_file_name: "test_db_set1.html".to_string(),
            title: Some("Test".to_string()),
            modification_date: None,
            summary: Some("Test".to_string()),
            tags: Some(original_tags.clone()),
            series: Some("Test".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };
        let ret = set(&mut conn, &article_with_tags1);

        assert!(ret.is_ok());

        match get_all_articles(&mut conn) {
            Ok(articles_with_tags) => {
                assert_eq!(articles_with_tags.len(), 1);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let res = get_tags_for_article(&mut conn, 1);
        assert!(res.is_ok());

        match res {
            Ok(tags_options) => match tags_options {
                Some(tags) => {
                    println!("tags: {:?}", tags);
                    assert_eq!(tags.len(), 2);
                }
                None => {
                    assert!(false);
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_db_set_middle() {
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

        let article_with_tags4 = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set4.mdwn".to_string(),
            dst_file_name: "test_db_set4.html".to_string(),
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

        set(&mut conn, &article_with_tags4).unwrap();

        let article_with_tags3_update = ArticleWithTags {
            id: None,
            src_file_name: "foo/bartest_db_set3.mdwn".to_string(),
            dst_file_name: "test_db_set3.html".to_string(),
            title: Some("Test2".to_string()),
            modification_date: None,
            summary: Some("Test2".to_string()),
            tags: Some(vec!["test2".to_string(), "test3".to_string()]),
            series: Some("Test3".to_string()),
            draft: Some(true),
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let ret = set(&mut conn, &article_with_tags3_update);
        match ret {
            Ok(db_reply) => {
                let assumed_result: HashSet<i32> = vec![4, 2].into_iter().collect();
                assert_eq!(db_reply.affected_articles, assumed_result);
            }
            Err(_) => {}
        }
    }
}
