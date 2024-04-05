use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::{FutureExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use salvo::prelude::*;
use salvo::websocket::{Message, WebSocket, WebSocketUpgrade};

type Users = RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, salvo::Error>>>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);

pub async fn on_connect (ws: WebSocket) {
  let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    tracing::info!("new chat user: {}", my_id);

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
        ONLINE_USERS.write().await.insert(my_id, tx);

        while let Some(result) = user_ws_rx.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={}): {}", my_id, e);
                    break;
                }
            };
            // user_message(my_id, msg).await;
        }

        // user_disconnected(my_id).await;
    };
    tokio::task::spawn(fut);

}

// async fn user_message(my_id: usize, msg: Message) {
//   let msg = if let Ok(s) = msg.to_str() {
//     s
//   } else {
//     return;
//   };

//   let new_msg = format!("<User#{}>: {}", my_id, msg);

//   // New message from this user, send it to everyone else (except same uid)...
//   for (&uid, tx) in ONLINE_USERS.read().await.iter() {
//     if my_id != uid {
//       if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
//         // The tx is disconnected, our `user_disconnected` code
//         // should be happening in another task, nothing more to
//         // do here.
//       }
//     }
//   }
// }