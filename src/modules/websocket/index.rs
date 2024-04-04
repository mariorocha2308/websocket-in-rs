use salvo::prelude::*;
use crate::modules::websocket::connect::index_connect;

#[handler]
async fn init_socket(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
  WebSocketUpgrade::new()
    .upgrade(req, res, index_connect)
    .await
}

pub fn websocket_router() -> Vec<Router> {
  vec![
    Router::with_path("/init").goal(init_socket),
  ]
}