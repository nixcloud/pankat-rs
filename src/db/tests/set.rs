use crate::db::tests::establish_connection;
use diesel::sqlite::SqliteConnection;
use crate::db::article::{get_all_articles, set};
use crate::db::initialize_schema;

use crate::articles::ArticleWithTags;
use crate::articles::NewArticle;

#[test]
fn test_db_set() {
    let mut conn: SqliteConnection = establish_connection();
    initialize_schema(&mut conn);
    let article_with_tags1 = ArticleWithTags {
        id: None,
        src_file_name: "foo/bartest_db_set.mdwn1".to_string(),
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

    set(&mut conn, &article_with_tags1);

    let article_with_tags2 = ArticleWithTags {
        id: None,
        src_file_name: "foo/bartest_db_set.mdwn2".to_string(),
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

    set(&mut conn, &article_with_tags2);

    let articles_with_tags: Vec<ArticleWithTags> = get_all_articles(&mut conn);
    assert_eq!(articles_with_tags.len(), 2);
}
