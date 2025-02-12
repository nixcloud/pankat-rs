use diesel::prelude::*;

use crate::db::schema::cache as cache_table;
use crate::db::schema::cache::dsl as cache_objects;

#[derive(Queryable, Insertable)]
#[diesel(belongs_to(Article))]
#[diesel(table_name = cache_table)]
pub struct Cache {
    pub id: Option<i32>,
    pub src_file_name: String,
    pub hash: String,
    pub html: String,
}

pub fn get_cache(conn: &mut SqliteConnection, src_file_name: String) -> QueryResult<Option<Cache>> {
    cache_objects::cache
        .filter(cache_objects::src_file_name.eq(src_file_name))
        .first(conn)
        .optional()
}

pub fn set_cache(
    conn: &mut SqliteConnection,
    src_file_name: String,
    hash: String,
    html: String,
) -> QueryResult<usize> {
    let new_cache = Cache {
        id: None,
        src_file_name: src_file_name,
        hash: hash,
        html: html,
    };

    diesel::insert_into(cache_table::table)
        .values(&new_cache)
        .execute(conn)
}
