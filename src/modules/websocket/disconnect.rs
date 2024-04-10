use uuid::Uuid;
use super::{connect::ONLINE_USERS, custom::notify_connections};

pub async fn disconnect(ws_id: Uuid) {
  ONLINE_USERS.write().await.remove(&ws_id);
  notify_connections(ws_id).await
}