use diesel::prelude::*;
use crate::db::index::establish_connection;
use crate::db::models::user::{IUser, User};
use crate::db::schema::users;

pub fn create_user() -> User {
  let conn = &mut establish_connection();

  let list_users = [
    IUser {
      username: "sboother0".to_string(),
      firstname: "Sarena".to_string(),
      email: "smallion0@github.io".to_string(),
      ip: "97.218.181.203".to_string()
    }, 
    IUser {
      username: "akloska1".to_string(),
      firstname: "Addy".to_string(),
      email: "alayzell1@washington.edu".to_string(),
      ip: "82.79.148.107".to_string()
    }, 
    IUser {
      username: "tcruz2".to_string(),
      firstname: "Tomaso".to_string(),
      email: "tpolin2@sfgate.com".to_string(),
      ip: "28.192.228.156".to_string()
    }, 
    IUser {
      username: "gblint3".to_string(),
      firstname: "Gustave".to_string(),
      email: "gwinton3@qq.com".to_string(),
      ip: "8.225.65.177".to_string()
    }, 
    IUser {
      username: "egiraux4".to_string(),
      firstname: "Emilie".to_string(),
      email: "eagnew4@marketwatch.com".to_string(),
      ip: "255.200.207.95".to_string()
    }, 
    IUser {
      username: "lpregel5".to_string(),
      firstname: "L;urette".to_string(),
      email: "lpetroff5@blogger.com".to_string(),
      ip: "217.174.184.229".to_string()
    }, 
    IUser {
      username: "laberdalgy6".to_string(),
      firstname: "Leslie".to_string(),
      email: "lcastiblanco6@sogou.com".to_string(),
      ip: "62.236.99.50".to_string()
    }, 
    IUser {
      username: "mbeldan7".to_string(),
      firstname: "Madalyn".to_string(),
      email: "mcastellino7@addthis.com".to_string(),
      ip: "62.243.27.34".to_string()
    }, 
    IUser {
      username: "wsnell8".to_string(),
      firstname: "Wyatt".to_string(),
      email: "wholgan8@mayoclinic.com".to_string(),
      ip: "36.193.254.192".to_string()
    }, 
    IUser {
      username: "rsteptow9".to_string(),
      firstname: "Rudolph".to_string(),
      email: "rgiorgini9@godaddy.com".to_string(),
      ip: "189.114.92.99".to_string()
    }
  ];

  diesel::insert_into(users::table)
    .values(&list_users)
    .returning(User::as_returning())
    .get_result(conn)
    .expect("Error saving new user collection")
}