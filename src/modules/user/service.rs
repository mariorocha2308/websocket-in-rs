use salvo::prelude::*;
use diesel::prelude::*;
use crate::db::{index::establish_connection, models::user::User, schema::users::dsl::users};

#[handler]
pub async fn get_users(res: &mut Response) {
  let connection = &mut establish_connection();

  let usersdata = users
    .select(User::as_select())
    .load(connection)
    .unwrap();

  res.render(Json(usersdata));
}