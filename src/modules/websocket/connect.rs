use uuid::Uuid;
use salvo::prelude::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap ;
use tokio::sync::{mpsc, RwLock};
use futures_util::{FutureExt, StreamExt};
use salvo::websocket::{Message, WebSocketUpgrade};
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::{custom::{notify_connections, handle_send_message}, disconnect::disconnect};

type Users = RwLock<HashMap<Uuid, mpsc::UnboundedSender<Result<Message, salvo::Error>>>>;
pub static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);

#[derive(Debug, Deserialize)]
pub struct UserConnection {
  pub _id: Uuid,
  pub nickname: String,
}

#[handler]
pub async fn handler_socket(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
  let queries = req.parse_queries::<UserConnection>();

  match queries {
    Ok(user) => WebSocketUpgrade::new().upgrade(req, res, move |ws| async move {
      // Split the socket into a sender and receive of messages.
      let (user_ws_tx, mut user_ws_rx) = ws.split();

      // Use an unbounded channel to handle buffering and flushing of messages
      // to the websocket...
      let (tx, rx) = mpsc::unbounded_channel();
      let rx = UnboundedReceiverStream::new(rx);
      let fut = rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
          tracing::error!(error = ?e, "websocket send error");
        }
      });
      tokio::task::spawn(fut);
      
      let fut = async move {
        ONLINE_USERS.write().await.insert(user._id, tx);
        notify_connections(user._id).await;

        while let Some(result) = user_ws_rx.next().await {
          let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
              eprintln!("websocket error(uid={}): {}", user._id, e);
              break;
            }
          };

          handle_send_message(user._id, user.nickname.clone(), msg).await;
        }

        disconnect(user._id).await;
      };
      tokio::task::spawn(fut);
    }).await,
    Err(_) => Err(StatusError::expectation_failed()),
  }
}