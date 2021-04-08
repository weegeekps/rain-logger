use rocket_contrib::json::Json;
use diesel::PgConnection;
use crate::DbConn;
use std::error::Error;
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Deserialize)]
struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: Uuid,
    valid_until: DateTime<Utc>
}

#[post("/auth/login", data = <payload>)]
pub fn login(conn: DbConn, payload: Json<LoginRequest>) -> Result<Json<LoginResponse>, Box<dyn Error>> {
    todo!("Implement login")
}

pub fn logout() {
    todo!("Implement logout")
}
