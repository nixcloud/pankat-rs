use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::path::Path;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod schema;
pub mod users;

pub fn establish_connection_pool() -> DbPool {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("sqlite:auth.db"));
    println!("Connecting to {}", database_url);
    // Ensure the database file exists
    if !Path::new("auth.db").exists() {
        println!("Creating SQLite database file...");
        std::fs::File::create("auth.db").expect("Failed to create SQLite database file");

        // Optionally initialize the schema here
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
