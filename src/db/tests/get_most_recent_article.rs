use crate::db::tests::establish_connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_get_most_recent_article() {
    let mut conn: SqliteConnection = establish_connection();
    // assert!(false);
}
