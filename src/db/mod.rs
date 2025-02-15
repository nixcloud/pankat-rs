use crate::config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::path::Path;
use std::path::PathBuf;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod article;
pub mod cache;
pub mod schema;
mod tests;
pub mod users;

pub fn establish_connection_pool() -> DbPool {
    let cfg = config::Config::get();
    let mut database_path = PathBuf::from(cfg.database.clone());
    database_path.push("pankat.sqlite");

    let database_url: &str = database_path.as_path().to_str().unwrap();

    println!("Connecting to {}", database_url);
    if !Path::new(database_url).exists() {
        println!("Creating SQLite database file...");
        std::fs::File::create(database_url).expect("Failed to create SQLite database file");
    }

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    initialize_schema(&mut db_pool.get().unwrap());

    db_pool
}

pub fn initialize_schema(connection: &mut SqliteConnection) {
    println!("Initializing schema...");
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
