//! fingerprints of the generated pages in the output directory, see src/fingerprint.rs
use crate::db::schema::outputs::dsl::*;
use diesel::prelude::*;

pub fn get(conn: &mut SqliteConnection, name: &str) -> Option<String> {
    outputs
        .filter(dst_file_name.eq(name))
        .select(fingerprint)
        .first(conn)
        .ok()
}

pub fn set(conn: &mut SqliteConnection, name: &str, value: &str) {
    let _ = diesel::replace_into(outputs)
        .values((dst_file_name.eq(name), fingerprint.eq(value)))
        .execute(conn);
}

pub fn delete(conn: &mut SqliteConnection, name: &str) {
    let _ = diesel::delete(outputs.filter(dst_file_name.eq(name))).execute(conn);
}
