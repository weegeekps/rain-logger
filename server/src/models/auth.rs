use std::error::Error;

use diesel::pg::PgConnection;
use log::debug;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use uuid::Uuid;

use crate::DbConn;
use crate::models::api_token::ApiToken;
use crate::models::user::User;
use crate::utils::jwt::read_jwt;

pub struct Auth {
    pub user: User,
    pub api_token: ApiToken,
}

impl Auth {
    pub fn new(conn: &PgConnection, token_id: Uuid) -> Result<Self, Box<dyn Error>> {
        let api_token = ApiToken::get(conn, token_id)?;
        let user = User::read(conn, api_token.user_id)?[0].clone();

        Ok(Auth {
            user,
            api_token,
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'a Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let auth_result: &Result<Auth, ()> = request.local_cache(|| {
            let conn: DbConn = match request.guard::<DbConn>().succeeded() {
                Some(d) => d,
                None => return Err(()),
            };

            let headers = request.headers();
            let jwts: Vec<_> = headers.get("Authorization").collect();
            if jwts.len() != 1 {
                return Err(());
            }

            let token_id = match read_jwt(&jwts[0][7..]) {
                Ok(t) => Uuid::parse_str(t.as_str()).unwrap(),
                Err(err) => {
                    debug!("{}", err.to_string());
                    return Err(());
                }
            };

            match Auth::new(&conn as &PgConnection, token_id) {
                Ok(auth) => Ok(auth),
                Err(_) => Err(())
            }
        });

        match auth_result.as_ref() {
            Ok(a) => {
                if a.api_token.force_invalid {
                    return Outcome::Failure((Status::Unauthorized, ()));
                }

                Outcome::Success(a)
            },
            Err(_) => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
