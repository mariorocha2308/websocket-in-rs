use short_uuid::ShortUuid;

pub async fn on_disconnect(conn_id: ShortUuid) {
  println!("good bye user: {conn_id}");
}