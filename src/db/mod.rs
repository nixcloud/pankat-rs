use crate::config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::path::Path;
use std::path::PathBuf;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod schema;
pub mod users;

pub fn establish_connection_pool() -> DbPool {
    let cfg = config::Config::get();
    let mut database_path = PathBuf::from(cfg.database.clone());

    database_path.push("pankat.sqlite");
    let database_url: &str = database_path.as_path().to_str().unwrap();

    println!("Connecting to {}", database_url);
    if !Path::new("pankat.db").exists() {
        println!("Creating SQLite database file...");
        std::fs::File::create("pankat.db").expect("Failed to create SQLite database file");
        initialize_schema(&database_url);
    }

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

fn initialize_schema(database_url: &str) {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    let mut connection =
        SqliteConnection::establish(database_url).expect("Failed to connect to SQLite database");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
