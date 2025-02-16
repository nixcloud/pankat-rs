use crate::db::tests::establish_connection_and_initialize_schema;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_db_get_special_pages() {
    let mut conn: SqliteConnection = establish_connection_and_initialize_schema();
    // assert!(false);
}
