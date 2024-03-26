use diesel::prelude::*;
// use chrono::Utc;
use crate::db::index::establish_connection;
use crate::db::models::user::{IUser, User};
use crate::db::schema::users;

pub fn create_user(new_user: IUser) -> User {
  let conn = &mut establish_connection();
  // let datetime = Utc::now();
  // let timestamp: i64 = datetime.timestamp();

  // let new_user = IUser {
  //   username: "Mario".to_string(),
  //   phone: "+52 920-312-3990".to_string(),
  //   password: "MkULZtZ8BzJta".to_string(),
  //   created_at: timestamp.to_string()
  // };

  diesel::insert_into(users::table)
    .values(&new_user)
    .returning(User::as_returning())
    .get_result(conn)
    .expect("Error saving new user collection")
}