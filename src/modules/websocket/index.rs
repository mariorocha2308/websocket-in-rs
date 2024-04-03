use salvo::prelude::*;
use super::service::{connect, index};

pub fn websocket_router() -> Vec<Router> {
  vec![
    Router::with_path("/").get(index).goal(connect),
  ]
}