mod del_by_id;
mod del_by_src_file_name;
mod get_all_articles;
mod get_all_series_from_visible_articles;
mod get_all_tags;
mod get_drafts;
mod get_most_recent_article;
mod get_prev_and_next_article;
mod get_prev_and_next_article_for_series;
mod get_special_pages;
mod get_visible_articles;
mod get_visible_articles_by_series;
mod get_visible_articles_by_tag;
mod set;

use crate::db::initialize_schema;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub fn establish_connection_and_initialize_schema() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:")
        .expect("Failed to create SQLite in-memory database");
    initialize_schema(&mut conn);
    conn
}

#[test]
fn test_diesel_in_memory_sqlite() {
    let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

    // Create a sample table
    diesel::sql_query("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&mut conn)
        .expect("Failed to create table");

    // Insert data
    diesel::sql_query("INSERT INTO test (name) VALUES ('Alice')")
        .execute(&mut conn)
        .expect("Failed to insert data");
}

#[test]
fn test_diesel_initialize_schema() {
    use crate::db::schema::tags::dsl as tags_objects;
    use crate::db::schema::tags::dsl::tags as tags_table;
    let mut conn: SqliteConnection = establish_connection_and_initialize_schema();

    // Check for table existence
    let result =
        diesel::sql_query("SELECT name FROM sqlite_master WHERE type='table' AND name='articles';")
            .execute(&mut conn);
    assert!(result.is_ok(), "Table 'articles' should exist.");

    // Insert data
    diesel::sql_query("INSERT INTO tags (name) VALUES ('linux')")
        .execute(&mut conn)
        .expect("Failed to insert data");

    let inserted_tag = tags_table
        .filter(tags_objects::id.eq(1))
        .load::<crate::db::article::Tag>(&mut conn);
    match inserted_tag {
        Ok(tags) => {
            assert_eq!(tags.len(), 1, "Expected one tag with name 'linux' and id 1");
            assert_eq!(
                tags[0].name,
                "linux".to_string(),
                "Tag name should be 'linux'"
            );
        }
        Err(_) => panic!("Failed to query the tags table"),
    }
}
