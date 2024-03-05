use salvo::prelude::*;
use super::service::get_users;

pub fn user_router() -> Vec<Router> {
  return vec![
    Router::with_path("/collection").get(get_users)
  ];
}