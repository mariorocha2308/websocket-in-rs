use salvo::prelude::*;
use bcrypt::{DEFAULT_COST, hash};
use crate::{db::{models::user::IblUser, ops::user::create_user}, ServerResponse};

#[handler]
pub async fn post_login(res: &mut Response, req: &mut Request) {
  let body = req.parse_body::<IblUser>().await.unwrap();
  // let connection = &mut establish_connection();

  // let users_collection: Vec<QblUser> = users
  //   .select(QblUser::as_select())
  //   .load(connection)
  //   .unwrap();

  res.render("login endpoint");
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

      let result = create_user(new_user);

      if result._id {
        res.status_code(StatusCode::CREATED).render(Json(ServerResponse{
          message: "User register successfully".to_string(),
          status_code: 201
        }));
      } else {
        res.status_code(StatusCode::BAD_GATEWAY).render(Json(ServerResponse{
          message: "User register successfully".to_string(),
          status_code: 502
        }));
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
