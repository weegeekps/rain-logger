use diesel::PgConnection;
use log::error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::DbConn;
use crate::models::api_token::ApiToken;
use crate::models::user::User;
use crate::utils::jwt::generate_jwt;

#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    jwt: String,
}

#[post("/auth/login", data = "<payload>")]
pub fn login(conn: DbConn, payload: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    let user_result = User::validate(&conn as &PgConnection, payload.name.to_string(), payload.password.to_string());

    // Generate token
    let api_token_result = match user_result {
        Ok(user) => ApiToken::create_for_user(&conn as &PgConnection, user),
        Err(err) => {
            // todo!("Detect the type of errors so we ignore bad login errors.")
            error!("{}", err.to_string());
            return Err(Status::Unauthorized);
        }
    };

    // todo!("Make this actually use a jwt.")
    let jwt_result = match api_token_result {
        Ok(api_token) => generate_jwt(api_token),
        Err(err) => {
            error!("{}", err.to_string());
            return Err(Status::InternalServerError);
        }
    };

    match jwt_result {
        Ok(jwt) => Ok(Json(LoginResponse { jwt })),
        Err(err) => {
            error!("{}", err.to_string());
            return Err(Status::InternalServerError);
        }
    }
}

pub fn logout() {
    todo!("Implement logout")
}
