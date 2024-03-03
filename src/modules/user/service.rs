use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct User {
  id: i16,
  name: String,
  email: String,
  firstname: String,
  ip_address: String
}

#[handler]
pub async fn get_users(res: &mut Response) {

  let users = [
    User {
      id: 1,
      name: String::from("htuley0"),
      firstname: String::from("Hildegarde"),
      email: String::from("himm0@fema.gov"),
      ip_address: String::from("7.135.125.20")
    }, 
    User {
      id: 2,
      name: String::from("cjaulmes1"),
      firstname: String::from("Chet"),
      email: String::from("csygrove1@gnu.org"),
      ip_address: String::from("169.149.124.123")
    }, 
    User {
      id: 3,
      name: String::from("atalks2"),
      firstname: String::from("Aurie"),
      email: String::from("amcelrath2@amazon.de"),
      ip_address: String::from("107.132.174.137")
    }, 
    User{
      id: 4,
      name: String::from("anatwick3"),
      firstname: String::from("Audie"),
      email: String::from("ajoll3@i2i.jp"),
      ip_address: String::from("200.250.72.189")
    }, 
    User {
      id: 5,
      name: String::from("tohoey4"),
      firstname: String::from("Tobe"),
      email: String::from("tdearsley4@xing.com"),
      ip_address: String::from("180.242.16.237")
    }
  ];

  res.render(Json(users));
}