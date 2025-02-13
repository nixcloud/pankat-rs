use sha2::{Digest, Sha256};

use crate::db::schema::cache as cache_table;
use crate::db::schema::cache::dsl as cache_objects;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(belongs_to(Article))]
#[diesel(table_name = cache_table)]
pub struct Cache {
    pub id: Option<i32>,
    pub src_file_name: String,
    pub hash: String,
    pub html: String,
}

pub fn get_cache(conn: &mut SqliteConnection, src_file_name: String) -> Option<Cache> {
    let v: QueryResult<Option<Cache>> = cache_objects::cache
        .filter(cache_objects::src_file_name.eq(src_file_name.clone()))
        .first(conn)
        .optional();
    match v {
        Ok(r) => r,
        Err(_) => {
            println!("Loading the cache for article: {} failed", src_file_name);
            None
        }
    }
}

pub fn set_cache(
    conn: &mut SqliteConnection,
    src_file_name: String,
    html: String,
) -> Result<(), String> {
    let mut hasher = Sha256::new();
    hasher.update(html.clone().as_bytes());
    let sha256_hash: String = format!("{:x}", hasher.finalize());

    let new_cache = Cache {
        id: None, // SQLite will auto-increment this if not provided
        src_file_name: src_file_name.clone(),
        hash: sha256_hash,
        html,
    };
    // println!(
    //     "Preparing to execute insert into cache table with data: {:#?}",
    //     new_cache
    // );

    match diesel::insert_into(cache_table::table)
        .values(&new_cache)
        .execute(conn)
    {
        Ok(rows) => {
            println!("Successfully inserted cache entry. Rows affected: {}", rows);
            Ok(())
        }
        Err(e) => {
            let error_message = format!(
                "Failed to insert cache entry for article {}: {:?}",
                src_file_name, e
            );
            println!("{}", error_message);
            Err(error_message) // Use detailed error message for debugging
        }
    }
}
