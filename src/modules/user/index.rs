use salvo::prelude::*;
use super::service::get_collection ;

pub fn user_router() -> Vec<Router> {
  vec![
    Router::with_path("/collection").get(get_collection),
  ]
}