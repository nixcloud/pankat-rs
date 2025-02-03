use crate::auth::UserLevel;
use crate::db::schema::users;
use diesel::prelude::*;

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
    use crate::db::schema::users::dsl::*;

    users
        .filter(username.eq(username_query))
        .first(conn)
        .optional()
}
