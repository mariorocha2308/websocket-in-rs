use salvo::prelude::*;
use super::service::{ post_login, post_register };

pub fn auth_router() -> Vec<Router> {
  vec![
    Router::with_path("/login").post(post_login),
    Router::with_path("/register").post(post_register)
  ]
}