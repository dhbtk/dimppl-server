use crate::database::AsyncConnection;
use crate::error_handling::AppResult;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, Insertable, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::models::User;
use crate::schema::users::dsl::users;

pub async fn create<'a>(user: &NewUser, conn: &mut AsyncConnection<'a>) -> AppResult<User> {
    Ok(diesel::insert_into(users::table())
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
        .await?)
}

pub async fn find_by_access_key<'a>(
    access_key: &str,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<User> {
    Ok(users
        .filter(crate::schema::users::access_key.eq(access_key))
        .select(User::as_select())
        .first(conn)
        .await?)
}

pub async fn find_one<'a>(id: i64, conn: &mut AsyncConnection<'a>) -> AppResult<User> {
    Ok(users
        .filter(crate::schema::users::id.eq(id))
        .select(User::as_select())
        .first(conn)
        .await?)
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    access_key: String,
}

impl Default for NewUser {
    fn default() -> Self {
        Self {
            access_key: generate_user_access_key(),
        }
    }
}

fn random_chars(count: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub fn generate_user_access_key() -> String {
    format!(
        "{}-{}-{}-{}-{}",
        random_chars(8),
        random_chars(4),
        random_chars(4),
        random_chars(10),
        random_chars(2)
    )
}
