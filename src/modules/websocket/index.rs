use salvo::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::connect::on_connect;

#[derive(Debug, Deserialize)]
pub struct UserConnection {
  pub _id: Uuid,
  pub nickname: String
}

pub fn websocket_router() -> Vec<Router> {
  vec![
    Router::with_path("/init").goal(handler_socket),
  ]
}

#[handler]
pub async fn handler_socket(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
  let queries = req.parse_queries::<UserConnection>();

  match queries {
    Ok(_) => {
      WebSocketUpgrade::new()
      .upgrade(req, res, on_connect)
      .await
    }
    Err(_) => {
      Err(StatusError::expectation_failed())
    }
  }
}