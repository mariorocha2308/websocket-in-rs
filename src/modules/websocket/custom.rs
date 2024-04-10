use uuid::Uuid;
use salvo::websocket::Message;
use super::connect::ONLINE_USERS;

pub async fn handle_send_message(ws_id: Uuid, nickname: String, msg: Message) {
  let msg = if let Ok(s) = msg.to_str() {
    s
  } else {
    return;
  };

  let new_msg = format!("<User::{}>: {}", nickname, msg);

  // New message from this user, send it to everyone else (except same uid)...
  for (&uid, tx) in ONLINE_USERS.read().await.iter() {
    if ws_id != uid {
      if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
        // The tx is disconnected, our `user_disconnected` code
        // should be happening in another task, nothing more to
        // do here.
      }
    }
  }
}

pub async fn notify_connections(ws_id: Uuid) {
  let users_online = ONLINE_USERS.read().await.keys()
    .filter_map(move |&key| 
      (key == ws_id).then_some(key.to_string()))
    .collect::<Vec<String>>();

  for (&uid, tx) in ONLINE_USERS.read().await.iter() {
    if ws_id != uid {
      if let Err(_disconnected) = tx.send( Ok(Message::text(serde_json::to_string(&users_online).unwrap()))) {
      }
    }
  }
}