#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate serde;

use dotenv::dotenv;

mod database;
mod schema;
mod models;
mod picard;



fn grab_env_vars() {
  dotenv().ok(); // Grabbing ENV vars
}

fn main() {
  grab_env_vars();

  let rocket = rocket::ignite().manage(database::create_db_pool());

  let rocket = picard::register_routes(rocket);
  
  rocket.launch();
}
