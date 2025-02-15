use crate::db::article::{get_all_articles, set};
use crate::db::tests::establish_connection;
use diesel::sqlite::SqliteConnection;

use crate::articles::ArticleWithTags;
use crate::articles::NewArticle;

#[test]
fn test_db_set() {
    let mut conn: SqliteConnection = establish_connection();

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

    let _ = set(&mut conn, &article_with_tags1);

    let article_with_tags2 = ArticleWithTags {
        id: None,
        src_file_name: "foo/bartest_db_set2.mdwn".to_string(),
        dst_file_name: "test_db_set2.html".to_string(),
        title: Some("Test2".to_string()),
        modification_date: None,
        summary: Some("Test2".to_string()),
        tags: Some(vec!["test2 test3".to_string()]),
        series: Some("Test2".to_string()),
        draft: None,
        special_page: None,
        timeline: None,
        anchorjs: None,
        tocify: None,
        live_updates: None,
    };

    let _ = set(&mut conn, &article_with_tags2);

    match get_all_articles(&mut conn) {
        Ok(result) => {
            let articles_with_tags: Vec<ArticleWithTags> = result;
            assert_eq!(articles_with_tags.len(), 2);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

#[test]
fn test_db_set_update() {
    let mut conn: SqliteConnection = establish_connection();
    //initialize_schema(&mut conn);

    let original_tags = vec!["test1 test2".to_string()];
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
        timeline: None,
        anchorjs: None,
        tocify: None,
        live_updates: None,
    };
    let _ = set(&mut conn, &article_with_tags1);

    let new_tags = vec!["test2 test3".to_string()];

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
        timeline: None,
        anchorjs: None,
        tocify: None,
        live_updates: None,
    };

    let _ = set(&mut conn, &article_with_tags2);

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
