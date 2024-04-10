use salvo::prelude::*;
use crate::modules::websocket::connect::handler_socket;

pub fn websocket_router() -> Vec<Router> {
  vec![
    Router::with_path("/init").goal(handler_socket),
  ]
}