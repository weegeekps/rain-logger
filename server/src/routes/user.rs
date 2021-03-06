use std::error::Error;

use diesel::PgConnection;
use rocket_contrib::json::Json;
use uuid::Uuid;

use crate::DbConn;
use crate::models::user::{User, UserJson};
use crate::models::auth::Auth;

#[get("/users")]
pub fn get_all_users(conn: DbConn, _auth: &Auth) -> Result<Json<Vec<UserJson>>, Box<dyn Error>> {
    let dirty_result = User::read_all(&conn as &PgConnection)?;
    let clean_result: Vec<UserJson> = dirty_result.into_iter().map(|u| UserJson::transform(&u)).collect();
    Ok(Json(clean_result))
}

#[get("/users/<id>")]
pub fn get_user(conn: DbConn, _auth: &Auth, id: String) -> Result<Json<UserJson>, Box<dyn Error>> {
    let dirty_result = User::read(&conn as &PgConnection, Uuid::parse_str(&id)?)?;
    let clean_result: Vec<UserJson> = dirty_result.into_iter().map(|u| UserJson::transform(&u)).collect();
    Ok(Json(clean_result[0].clone()))
}
