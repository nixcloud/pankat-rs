use crate::config;
use colored::Colorize;
use diesel::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use diesel::r2d2::{self, ConnectionManager};
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

    let c = format!("Connecting to {}", database_url);
    println!("{}", c.yellow());
    if !Path::new(database_url).exists() {
        println!("Creating SQLite database file...");
        std::fs::File::create(database_url).expect("Failed to create SQLite database file");
    }

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool");
    initialize_schema(&mut pool.get().unwrap());

    pool
}

pub fn initialize_schema(connection: &mut SqliteConnection) {
    println!("Checking & doing schema updates...");
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
