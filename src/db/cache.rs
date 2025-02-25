use crate::db::schema;
use crate::db::schema::cache::dsl as cache_objects;
use crate::db::schema::cache::dsl::cache as cache_table;

use diesel::prelude::*;

#[derive(Queryable, Insertable, Clone, Debug, AsChangeset)]
#[diesel(belongs_to(Article))]
#[diesel(table_name = schema::cache)]
pub struct Cache {
    pub id: Option<i32>,
    pub src_file_name: String,
    pub hash: String,
    pub html: String,
}

pub fn get_cache_src_file_names(
    conn: &mut SqliteConnection,
) -> Result<Vec<(Option<i32>, String)>, diesel::result::Error> {
    let res: QueryResult<Vec<(Option<i32>, String)>> = cache_table
        .select((cache_objects::id, cache_objects::src_file_name))
        .load::<(Option<i32>, String)>(conn);
    match res {
        Ok(r) => Ok(r),
        Err(e) => {
            println!("get_cache_src_file_names: Loading all 'src_file_name' from cache failed");
            Err(e)
        }
    }
}

pub fn del_cache_by_id(conn: &mut SqliteConnection, id: i32) -> Result<(), diesel::result::Error> {
    let ret = diesel::delete(cache_table.filter(cache_objects::id.eq(id))).execute(conn);

    match ret {
        Ok(r) => {
            if r == 0 {
                println!("Cache entry with id {} not found", id);
            }
            Ok(())
        }
        Err(e) => {
            println!("Error deleting cache entry with id {}: {}", id, e);
            Err(e)
        }
    }
}

// FIXME rewrite to Result
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

pub fn compute_hash(html: String) -> String {
    use std::hash::Hasher;
    use twox_hash::XxHash64;

    let mut hasher = XxHash64::default();
    hasher.write(html.as_bytes());
    let xxhash64_hash = hasher.finish();
    format!("{:x}", xxhash64_hash)
}

pub fn set_cache(
    conn: &mut SqliteConnection,
    src_file_name: String,
    html: String,
    hash: String,
) -> Result<(), String> {
    let new_cache = Cache {
        id: None, // SQLite will auto-increment this if not provided
        src_file_name: src_file_name.clone(),
        hash: hash,
        html: html.clone(),
    };

    // Check if the cache entry already exists
    let existing_cache: QueryResult<Option<Cache>> = cache_objects::cache
        .filter(cache_objects::src_file_name.eq(src_file_name.clone()))
        .first(conn)
        .optional();

    match existing_cache {
        Ok(Some(_)) => {
            // Update the existing cache entry
            match diesel::update(
                cache_objects::cache.filter(cache_objects::src_file_name.eq(src_file_name.clone())),
            )
            .set(&new_cache)
            .execute(conn)
            {
                Ok(_rows) => {
                    //println!("Successfully updated cache entry. Rows affected: {}", rows);
                    Ok(())
                }
                Err(e) => {
                    let error_message = format!(
                        "Failed to update cache entry for article {}: {:?}",
                        src_file_name, e
                    );
                    println!("{}", error_message);
                    Err(error_message) // Use detailed error message for debugging
                }
            }
        }
        Ok(None) => {
            // Insert new cache entry if it does not exist
            match diesel::insert_into(cache_table)
                .values(&new_cache)
                .execute(conn)
            {
                Ok(_) => {
                    //println!("Successfully inserted cache entry. Rows affected: {}", rows);
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
        Err(e) => {
            let error_message = format!(
                "Failed to check existing cache for article {}: {:?}",
                src_file_name, e
            );
            println!("{}", error_message);
            Err(error_message)
        }
    }
}
