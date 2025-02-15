use crate::db::tests::establish_connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_get_special_pages() {
    let mut conn: SqliteConnection = establish_connection();
    // assert!(false);
}
