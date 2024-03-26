use diesel::prelude::*;
use diesel::sql_types::Timestamp;
use crate::db::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: String,
  pub username: String,
  pub phone: String,
  pub password: String,
  pub created_at: Timestamp
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = users)]
pub struct IUser {
  pub username: String,
  pub phone: String,
  pub password: String,
}