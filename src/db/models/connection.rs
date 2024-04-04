use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::schema::connections;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = connections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QblConnection {
  pub conn_id: String,
  pub nickname: String,
  pub user_ref: Uuid,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = connections)]
pub struct IblConnection {
  pub conn_id: String,
  pub nickname: String,
  pub user_ref: Uuid,
}