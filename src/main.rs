pub mod modules;
pub mod db;

use db::models::user::User;
use salvo::prelude::*;
use salvo::cors::Cors;
use salvo::http::Method;
use modules::route::router;

// ----------------------------------------------
use diesel::prelude::*;
use db::index::establish_connection;
use db::ops::user::create_user;
use crate::db::schema::users::dsl::users;
// ----------------------------------------------

#[handler]
async fn index(res: &mut Response) {
  res.render("Welcome to Salvo application!")
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt().init();
  
  // CONFIG CORS
  let cors = Cors::new()
  .allow_origin("*")
  .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
  .into_handler();

  let mut app_routing = router();
  let router = Router::new().path("/api/v1").get(index).append(&mut app_routing);
  let service = Service::new(router).hoop(cors);
  let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

  // PRELOAD COLLECTION OF USERS WHEN RUN SERVER
  let connection = &mut establish_connection();

  let users_collection: Vec<User> = users
    .select(User::as_select())
    .load(connection)
    .unwrap();

  if users_collection.len() == 0 {
    create_user();
  }

  // RUNNING SERVER
  Server::new(acceptor).serve(service).await;
}