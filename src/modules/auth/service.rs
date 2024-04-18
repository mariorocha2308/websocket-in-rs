use salvo::prelude::*;
use serde_json::json;

use bcrypt::{hash, verify, DEFAULT_COST};
use crate::{db::{models::user::IblUser, ops::user::{create_user, get_user_by_nickname}}, ServerResponse};

#[handler]
pub async fn post_login(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<IblUser>().await;

  match body {
    Ok(login) => {
      let handle_user = get_user_by_nickname(login.nickname);

      match handle_user {
        Ok(user) => {
          let verify_pass = verify(login.keypass, &user.keypass);

          match verify_pass {
            Ok(res_verify) => {
              if res_verify {
                res.status_code(StatusCode::ACCEPTED).render(Json(ServerResponse{
                  message: "Success: Valid authorization".to_string(),
                  status_code: 202, 
                  data: Some(serde_json::from_value(json!({
                    "_id": user._id,
                    "nickname": user.nickname
                  })).unwrap())
                }));
              } else {
                res.status_code(StatusCode::UNAUTHORIZED).render(Json(ServerResponse{
                  message: "Error: Invalid password".to_string(),
                  status_code: 401, 
                  data: None
                }));
              }
            }
            Err(_) => {
              res.status_code(StatusCode::UNAUTHORIZED).render(Json(ServerResponse{
                message: "Error: Bcrypt does not work".to_string(),
                status_code: 401, 
                data: None
              }));
            }
          }
        }
        Err(_) => {
          res.status_code(StatusCode::NOT_FOUND).render(Json(ServerResponse{
            message: "Error: Trying to find item in database".to_string(),
            status_code: 404,
            data: None
          }));
        }
      }
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

#[handler]
pub async fn post_register(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<IblUser>().await;

  match body {
    Ok(user) => {
      let hashed_pass = hash(user.keypass, DEFAULT_COST).unwrap();

      let new_user = IblUser { 
        nickname: user.nickname.to_string(), 
        telephone: Some(user.telephone.unwrap()), 
        keypass: hashed_pass.to_string()
      };

      let handle_create = create_user(new_user);

      match handle_create {
        Ok(user) => {

          res.status_code(StatusCode::CREATED).render(Json(ServerResponse{
            message: "Success: User register successfully".to_string(),
            status_code: 201,
            data: Some(serde_json::from_value(json!({
              "_id": user._id
            })).unwrap())
          }));
        }
        Err(_) => {
          res.status_code(StatusCode::BAD_GATEWAY).render(Json(ServerResponse{
            message: "Error: Set data in database".to_string(),
            status_code: 502,
            data: None
          }));
        }
      }
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
