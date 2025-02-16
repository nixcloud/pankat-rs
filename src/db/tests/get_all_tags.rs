use crate::db::article::{get_all_articles, get_drafts, set};
use crate::db::initialize_schema;
use crate::db::tests::establish_connection_and_initialize_schema;
use diesel::sqlite::SqliteConnection;

use crate::articles::ArticleWithTags;
use crate::articles::NewArticle;

#[test]
fn test_db_get_all_tags() {
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

    let ret = set(&mut conn, &article_with_tags1);
    assert!(ret.is_ok());

    match get_drafts(&mut conn) {
        Ok(result) => {
            let articles_with_tags: Vec<ArticleWithTags> = result;
            assert_eq!(articles_with_tags.len(), 0);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let article_with_tags2 = ArticleWithTags {
        id: None,
        src_file_name: "foo/bartest_db_set2.mdwn".to_string(),
        dst_file_name: "test_db_set2.html".to_string(),
        title: Some("Test2".to_string()),
        modification_date: None,
        summary: Some("Test2".to_string()),
        tags: Some(vec!["test2 test3".to_string()]),
        series: Some("Test2".to_string()),
        draft: Some(true),
        special_page: None,
        timeline: None,
        anchorjs: None,
        tocify: None,
        live_updates: None,
    };

    let ret = set(&mut conn, &article_with_tags2);
    assert!(ret.is_ok());
    
    match get_drafts(&mut conn) {
        Ok(result) => {
            let articles_with_tags: Vec<ArticleWithTags> = result;
            assert_eq!(articles_with_tags.len(), 1);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
