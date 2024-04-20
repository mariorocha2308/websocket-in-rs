use salvo::prelude::*;
use serde_json::json;
use std::collections::HashMap;

use crate::{db::ops::user::get_all_users, ServerResponse};

#[handler]
pub async fn get_collection(res: &mut Response) {
  let handle_get = get_all_users();

  match handle_get {
    Ok(collection) => {
      let mut collect_users = HashMap::new();

      collect_users.insert(
        "users".to_string(),
        json!(collection)
      );

      res.status_code(StatusCode::OK).render(Json(ServerResponse{
        message: "Success: User collection".to_string(),
        status_code: 202, 
        data: Some(collect_users)
      }));
    }
    Err(err) => {
      res.status_code(StatusCode::EXPECTATION_FAILED).render(Json(ServerResponse{
        message: err.to_string(),
        status_code: 417,
        data: None
      }));
    }
  }
}
