use salvo::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db::{models::user::IblUser, ops::user::{create_user, get_user_by_nickname}}, ServerResponse};

// TODO: CREATE OPTIONAL FIELD TELEPHONE IN STRUCT "IblUser"
#[derive(Deserialize)]
struct UserLogin {
  nickname: String,
  keypass: String
}

#[derive(Serialize)]
struct Session {
  _id: Uuid,
  nickname: String
}

#[handler]
pub async fn post_login(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<UserLogin>().await;

  match body {
    Ok(login) => {
      let handle_user = get_user_by_nickname(login.nickname);

      match handle_user {
        Ok(user) => {
          let verify_pass = verify(login.keypass, &user.keypass);
          
          match verify_pass {
            Ok(_) => {
              res.status_code(StatusCode::ACCEPTED).render(Json(Session{
                _id: user._id,
                nickname: user.nickname
              }));
            }
            Err(_) => {
              res.status_code(StatusCode::UNAUTHORIZED).render(Json(ServerResponse{
                message: "Error: Invalid Credentials".to_string(),
                status_code: 401
              }));
            }
          }
        }
        Err(_) => {
          res.status_code(StatusCode::NOT_FOUND).render(Json(ServerResponse{
            message: "Error: Trying to find item in database".to_string(),
            status_code: 404
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
            message: "Success: User register successfully".to_string(),
            status_code: 201
          }));
        }
        Err(_) => {
          res.status_code(StatusCode::BAD_GATEWAY).render(Json(ServerResponse{
            message: "Error: Set data in database".to_string(),
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
