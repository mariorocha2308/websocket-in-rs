use salvo::prelude::*;
use bcrypt::{DEFAULT_COST, hash};
use crate::{db::{models::user::IblUser, ops::user::create_user}, ServerResponse};

#[handler]
pub async fn post_login(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<IblUser>().await;

  match body {
    Ok(_) => {
    }

    Err(err) => {
      res.status_code(StatusCode::EXPECTATION_FAILED).render(Json(ServerResponse{
        message: err.to_string(),
        status_code: 417
      }));
    }
  }
}

#[handler]
pub async fn post_register(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<IblUser>().await;

  match body {
    Ok(user) => {
      let hashed_pass = hash(user.keypass, DEFAULT_COST).unwrap();

      let new_user = IblUser { 
        nickname: user.nickname.to_string(), 
        telephone: user.telephone.to_string(), 
        keypass: hashed_pass.to_string()
      };

      let handle_create = create_user(new_user);

      match handle_create {
        Ok(_) => {
          res.status_code(StatusCode::CREATED).render(Json(ServerResponse{
            message: "User register successfully".to_string(),
            status_code: 201
          }));
        }
        Err(_) => {
          res.status_code(StatusCode::BAD_GATEWAY).render(Json(ServerResponse{
            message: "Error set data in database".to_string(),
            status_code: 502
          }));
        }
      }
    }

    Err(err) => {
      res.status_code(StatusCode::EXPECTATION_FAILED).render(Json(ServerResponse{
        message: err.to_string(),
        status_code: 417
      }));
    }
  }
}
