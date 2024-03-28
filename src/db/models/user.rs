use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::db::schema::users;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QblUser {
  pub _id: Uuid,
  pub nickname: String,
  pub telephone: String,
  pub keypass: String,
  pub created_at: DateTime<Utc>
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct IblUser {
  pub nickname: String,
  pub telephone: Option<String>,
  pub keypass: String,
}