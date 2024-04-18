pub mod modules;
pub mod db;

use salvo::prelude::*;
use salvo::cors::{Cors, Any};
use salvo::http::Method;
use modules::route::router;
use std::collections::HashMap;
use salvo::logging::Logger;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct ServerResponse {
  message: String,
  status_code: i32,
  data: Option<HashMap<String, serde_json::Value>>
}

#[handler]
async fn index(res: &mut Response) {
  res.status_code(StatusCode::OK).render("Welcome to Salvo application!")
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt().init();
  
  // CONFIG CORS
  let cors = Cors::new()
    .allow_origin(Any)
    .allow_headers(Any)
    .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .into_handler();

  let mut app_routing = router();
  let router = Router::new().path("/api/v1").get(index).append(&mut app_routing);
  let service = Service::new(router).hoop(cors).hoop(Logger::new());
  let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

  // RUNNING SERVER
  Server::new(acceptor).serve(service).await;
}