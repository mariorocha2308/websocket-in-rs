use diesel::sql_types::Uuid;
use salvo::prelude::*;

use crate::db::{models::user::{IUser, User}, ops::user::create_user};
// use diesel::prelude::*;
// use crate::db::{index::establish_connection, models::user::User, schema::users::dsl::users};
use bcrypt::{DEFAULT_COST, hash};

#[handler]
pub async fn post_login(res: &mut Response) {
  // let connection = &mut establish_connection();
  
  
  // let valid = verify("hunter2", &hashed);
  res.render("login endpoint");
}

#[handler]
pub async fn post_register(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<User>().await.unwrap();
  let hashed_pass = hash(body.password, DEFAULT_COST).unwrap();
  
  let datetime = Utc::now();
  let id = Uuid::new_v4();
  let timestamp: i64 = datetime.timestamp();

  create_user();
  // println!("{body:?}");


  res.render(Json(body));
}
