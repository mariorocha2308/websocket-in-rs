use diesel::{result::Error, prelude::*};
use crate::db::index::establish_connection;
use crate::db::models::user::{IblUser, QblUser};
use crate::db::schema::users::{self, nickname};

pub fn create_user(new_user: IblUser) -> Result<QblUser, Error> {
  let conn = &mut establish_connection();

  diesel::insert_into(users::table)
    .values(&new_user)
    .returning(QblUser::as_returning())
    .get_result(conn)
}

pub fn get_user_by_nickname(name: String) -> Result<QblUser, Error> {
  let conn = &mut establish_connection();

  return users::table
  .filter(nickname.eq(name))
  .select(QblUser::as_select())
  .first(conn)
}

//* THIS REQUEST DB HAVE A CONFLICT SECURITY: RETURN ALL FIELDS OF USER TABLE LIKE: KEYPASS
pub fn get_all_users() -> Result<Vec<QblUser>, Error> {
  let conn = &mut establish_connection();

  return users::table
    .select(QblUser::as_select())
    .load::<QblUser>(conn)
}