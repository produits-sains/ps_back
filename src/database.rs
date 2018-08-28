extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;
extern crate dotenv;

use std::env;
use self::r2d2::Pool;
use self::r2d2_diesel::ConnectionManager;
use self::diesel::prelude::*;

pub fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
  println!("env {}", env::var("DATABASE_URL").unwrap_or("DATABASE_URL must be set".to_string()));
  // Pull DATABASE_URL env var
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  // Create a connection pool manager for a Sqlite connection at the `database_url`
  let manager = ConnectionManager::<SqliteConnection>::new(database_url);

  // Create the pool with the default config and the r2d2_diesel connection manager
  Pool::new(manager).expect("Failed to create pool.")
}

#[test]
fn it_should_load_test_database() {
  dotenv::from_filename("tests/env_testdatabase").ok();
  create_db_pool();
}
