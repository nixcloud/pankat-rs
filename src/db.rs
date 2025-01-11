use crate::auth::UserLevel;
use crate::schema::users;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::path::Path;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub level: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub level: &'a str,
}

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

pub fn create_user(
    conn: &mut SqliteConnection,
    username: &str,
    password: &str,
    level: UserLevel,
) -> QueryResult<i32> {
    println!("create_user: {}, {}, {:?}", username, password, level);

    let level_str = format!("{:?}", level);
    let new_user = NewUser {
        username,
        password,
        level: &level_str,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    users::table
        .select(users::id)
        .order(users::id.desc())
        .first(conn)
}

pub fn get_user_by_username(
    conn: &mut SqliteConnection,
    username_query: &str,
) -> QueryResult<Option<User>> {
    use crate::schema::users::dsl::*;

    users
        .filter(username.eq(username_query))
        .first(conn)
        .optional()
}
