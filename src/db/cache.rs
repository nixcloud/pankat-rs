use crate::db::schema::cache;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(belongs_to(Article))]
#[diesel(table_name = cache)]
pub struct Cache {
    pub id: i32,
    pub src_file_name: String,
    pub hash: String,
    pub html: String,
}

// pub fn set(conn: &mut SqliteConnection, src_file_name: String, hash: String, html: String) -> QueryResult<i32> {
//     diesel::insert_into(cache::table)
//         .values(new_article)
//         .execute(conn)?;

//     articles::table
//         .select(articles::id)
//         .order(articles::id.desc())
//         .first(conn)
// }
