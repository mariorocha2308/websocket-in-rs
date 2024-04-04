pub async fn index_disconnect(my_id: usize) {
  println!("good bye user: {}", my_id);
  // Stream closed up, so remove from the user list
  // ONLINE_USERS.write().await.remove(&my_id);
}