use diesel::PgConnection;
use log::{debug, error};
use rocket::http::Status;
use rocket_contrib::json::Json;
use uuid::{Error, Uuid};

use crate::DbConn;
use crate::models::auth::Auth;
use crate::models::precipitation_log::PrecipitationLog;

#[get("/logs/entries")]
pub fn get_all_entries(conn: DbConn, _auth: &Auth) -> Result<Json<Vec<PrecipitationLog>>, Status> {
    match PrecipitationLog::read_all(&conn as &PgConnection) {
        Ok(entries) => Ok(Json(entries)),
        Err(err) => {
            error!("{}", err.to_string());
            Err(Status::InternalServerError)
        }
    }
}

#[get("/logs/entry/<id>")]
pub fn get_entry(conn: DbConn, _auth: &Auth, id: String) -> Result<Json<PrecipitationLog>, Status> {
    let parsed_id = match Uuid::parse_str(id.as_str()) {
        Ok(id) => id,
        Err(err) => {
            debug!("{}", err.to_string());
            return Err(Status::BadRequest);
        }
    };

    match PrecipitationLog::read(&conn as &PgConnection, parsed_id) {
        Ok(entry) => match entry {
            Some(e) => Ok(Json(e)),
            None => Err(Status::NotFound),
        },
        Err(err) => {
            error!("{}", err.to_string());
            Err(Status::InternalServerError)
        }
    }
}

#[post("/logs/entry", data = "<entry>")]
pub fn create_entry(conn: DbConn, _auth: &Auth, entry: Json<PrecipitationLog>) -> Result<Json<PrecipitationLog>, Status> {
    todo!("implement create_entry")
}

#[put("/logs/entry/<id>", data = "<entry>")]
pub fn update_entry(conn: DbConn, _auth: &Auth, id: String, entry: Json<PrecipitationLog>) -> Result<Json<PrecipitationLog>, Status> {
    let parsed_id = match Uuid::parse_str(id.as_str()) {
        Ok(id) => id,
        Err(err) => {
            debug!("{}", err.to_string());
            return Err(Status::BadRequest);
        }
    };

    if entry.id != parsed_id {
        return Err(Status::BadRequest);
    }

    match PrecipitationLog::upsert(&conn as &PgConnection, &entry) {
        Ok(e) => Ok(Json(e)),
        Err(err) => {
            error!("{}", err.to_string());
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/logs/entry/<id>")]
pub fn delete_entry(conn: DbConn, _auth: &Auth, id: String) -> Result<Status, Status> {
    todo!("implement delete_entry")
}
