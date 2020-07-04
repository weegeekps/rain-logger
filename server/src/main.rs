#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate uuid;

pub mod schema;
pub mod models;

#[get("/health")]
fn health_check() -> &'static str {
    "ok"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            health_check,
        ])
        .launch();
}