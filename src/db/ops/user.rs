use diesel::prelude::*;
use crate::db::models::user::{IUser, User};
use crate::db::schema::users;

pub fn create_user(conn: &mut PgConnection) -> User {

  let list_users = [
    IUser {
      username: String::from("Mario"),
      firstname: String::from("Rocha"),
      email: String::from("virgoroch852@gmail.com"),
      ip: String::from("1.123.123.12")
    }
  ];

  diesel::insert_into(users::table)
    .values(&list_users)
    .returning(User::as_returning())
    .get_result(conn)
    .expect("Error saving new post")
}