use salvo::prelude::*;

use super::auth::index::auth_router;
use super::websocket::index::websocket_router;

pub fn router() -> Vec<Router> {
  vec![
    Router::with_path("/auth").append(&mut auth_router()),
    Router::with_path("/websocket").append(&mut websocket_router()),
  ]
}