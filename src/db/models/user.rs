use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::users;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub username: String,
  pub firstname: String,
  pub email: String,
  pub ip: String
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct IUser {
  pub username: String,
  pub firstname: String,
  pub email: String,
  pub ip: String
}