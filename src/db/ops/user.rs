use diesel::prelude::*;
use crate::db::index::establish_connection;
use crate::db::models::user::{IblUser, QblUser};
use crate::db::schema::users;

pub fn create_user(new_user: IblUser) -> QblUser {
  let conn = &mut establish_connection();

  diesel::insert_into(users::table)
    .values(&new_user)
    .returning(QblUser::as_returning())
    .get_result(conn)
    .expect("Error saving new user collection")
}