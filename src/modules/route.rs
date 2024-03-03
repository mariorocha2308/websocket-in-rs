use salvo::prelude::*;
use super::user::index::user_router;

pub fn router() -> Vec<Router> {
  let routers = vec![
    Router::with_path("/users").append(&mut user_router()),
  ];

  return routers;
}