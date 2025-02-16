use crate::db::tests::establish_connection_and_initialize_schema;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_db_get_most_recent_article() {
    let mut conn: SqliteConnection = establish_connection_and_initialize_schema();
    // assert!(false);
}
