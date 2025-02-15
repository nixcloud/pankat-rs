use crate::db::tests::establish_connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_db_del() {
    let mut conn: SqliteConnection = establish_connection();
    // assert!(false);
}
