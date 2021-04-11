#![feature(proc_macro_hygiene, decl_macro)]
extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use dotenv::dotenv;

pub mod schema;
pub mod models;
pub mod routes;
pub mod utils;

#[get("/health")]
fn health_check() -> &'static str {
    "ok"
}

#[database("rain_logger_db")]
pub struct DbConn(diesel::PgConnection);

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/api", routes![
            health_check,
            routes::auth::login,
            routes::auth::logout,
            routes::user::get_all_users,
            routes::user::get_user,
            routes::log::get_all_entries,
            routes::log::get_entry,
            routes::log::create_entry,
            routes::log::update_entry,
            routes::log::delete_entry,
        ])
        .launch();
}
