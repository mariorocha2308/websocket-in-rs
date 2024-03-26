use salvo::prelude::*;
use super::auth::index::auth_router;

pub fn router() -> Vec<Router> {
  vec![
    Router::with_path("/auth").append(&mut auth_router()),
  ]
}