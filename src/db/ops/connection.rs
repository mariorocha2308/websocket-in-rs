use diesel::{result::Error, prelude::*};
use crate::db::index::establish_connection;
use crate::db::models::connection::{IblConnection, QblConnection};
use crate::db::schema::connections::{self, conn_id};

pub fn create_connection(new_connection: IblConnection) -> Result<QblConnection, Error> {
  let conn = &mut establish_connection();

  diesel::insert_into(connections::table)
    .values(&new_connection)
    .returning(QblConnection::as_returning())
    .get_result(conn)
}

pub fn remove_connection_by_id(id: String) -> Result<QblConnection, Error> {
  let conn = &mut establish_connection();

  diesel::delete(connections::table)
  .filter(conn_id.like(id))
  .get_result(conn)
}