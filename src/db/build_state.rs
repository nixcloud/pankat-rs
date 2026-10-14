//! the global build inputs of the last run, see fingerprint::report_changes
use crate::db::schema::build_state::dsl::*;
use diesel::prelude::*;

pub fn get(conn: &mut SqliteConnection, name: &str) -> Option<String> {
    build_state
        .filter(key.eq(name))
        .select(value)
        .first(conn)
        .ok()
}

pub fn set(conn: &mut SqliteConnection, name: &str, new_value: &str) {
    let _ = diesel::replace_into(build_state)
        .values((key.eq(name), value.eq(new_value)))
        .execute(conn);
}

/// --rebuild: forget all fingerprints, so every article is parsed, every page written again
/// and pandoc runs for all of them
pub fn forget_fingerprints(conn: &mut SqliteConnection) -> QueryResult<()> {
    use crate::db::schema::{articles, cache, outputs};
    diesel::update(articles::table)
        .set(articles::hash.eq(""))
        .execute(conn)?;
    diesel::delete(cache::table).execute(conn)?;
    diesel::delete(outputs::table).execute(conn)?;
    Ok(())
}
