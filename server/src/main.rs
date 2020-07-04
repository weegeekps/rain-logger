#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate uuid;
extern crate chrono;
extern crate bcrypt;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod routes;

use dotenv::dotenv;

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
            routes::user::get_all_users,
            routes::user::get_user
        ])
        .launch();
}
