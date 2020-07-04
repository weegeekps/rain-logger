use crate::DbConn;
use rocket_contrib::json::Json;
use crate::models::user::{User, UserJson};
use std::error::Error;
use uuid::Uuid;

#[get("/users")]
pub fn get_all_users(conn: DbConn) -> Result<Json<Vec<UserJson>>, Box<dyn Error>> {
    let dirty_result = User::read_all(&conn)?;
    let clean_result: Vec<UserJson> = dirty_result.into_iter().map(|u| UserJson::transform(&u)).collect();
    Ok(Json(clean_result))
}

#[get("/users/<id>")]
pub fn get_user(conn: DbConn, id: String) -> Result<Json<UserJson>, Box<dyn Error>> {
    let dirty_result = User::read(&conn, Uuid::parse_str(&id)?)?;
    let clean_result: Vec<UserJson> = dirty_result.into_iter().map(|u| UserJson::transform(&u)).collect();
    Ok(Json(clean_result[0].clone()))
}
