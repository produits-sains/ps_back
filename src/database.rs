extern crate diesel;
extern crate dotenv;

use std::env;
use std::ops::Deref;
use self::diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use self::diesel::prelude::*;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};

pub fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
  println!("env {}", env::var("DATABASE_URL").unwrap_or("DATABASE_URL must be set".to_string()));
  // Pull DATABASE_URL env var
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  // Create a connection pool manager for a Sqlite connection at the `database_url`
  let manager = ConnectionManager::<SqliteConnection>::new(database_url);

  // Create the pool with the default config and the r2d2_diesel connection manager
  Pool::new(manager).expect("Failed to create pool.")
}

pub struct DbConn(PooledConnection<ConnectionManager<SqliteConnection>>);


impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = (); // Associated type, we must have an error we can `Debug`

   fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<SqliteConnection>>>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}


impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
