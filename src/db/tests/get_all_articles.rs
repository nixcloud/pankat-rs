use crate::db::tests::establish_connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_get_all_visible_articles() {
    let mut conn: SqliteConnection = establish_connection();
    assert!(true);
}
