use salvo::prelude::*;
use super::user::index::user_router;

pub fn router() -> Vec<Router> {
  vec![
    Router::with_path("/users").append(&mut user_router()),
  ]
}