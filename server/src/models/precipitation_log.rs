use std::error::Error;

use chrono::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::precipitation_logs;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PrecipitationType {
    Unidentified = 0,
    Liquid = 1,
    Freezing = 2,
    Frozen = 3,
}

impl PrecipitationType {
    // So dumb that casting enums still isn't a thing without a crate or code like this.
    pub fn from_i16(value: i16) -> Self {
        match value {
            1 => PrecipitationType::Liquid,
            2 => PrecipitationType::Freezing,
            3 => PrecipitationType::Frozen,
            _ => PrecipitationType::Unidentified,
        }
    }
}

#[derive(Serialize, Deserialize, Identifiable, AsChangeset, Queryable, Insertable, Clone)]
#[table_name = "precipitation_logs"]
pub struct PrecipitationLog {
    pub id: Uuid,
    pub measurement: f32,
    pub logged_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub ptype: i16,
    pub anomaly: bool,

    #[serde(skip_serializing, skip_deserializing)]
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl PrecipitationLog {
    pub fn new(measurement: f32, logged_at: DateTime<Utc>, ptype: PrecipitationType, notes: Option<String>, anomaly: bool) -> Self {
        let integer_type = ptype as i16;

        Self {
            id: Uuid::new_v4(),
            measurement,
            logged_at,
            notes,
            ptype: integer_type,
            anomaly,
            deleted: false,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }

    fn new_for_upsert(db_entry: &Self, user_entry: &Self) -> Self {
        Self {
            id: db_entry.id.clone(),
            measurement: user_entry.measurement,
            logged_at: user_entry.logged_at,
            notes: user_entry.notes.to_owned(),
            ptype: user_entry.ptype,
            anomaly: user_entry.anomaly,
            deleted: false,
            created_at: db_entry.created_at,
            modified_at: Utc::now(),
        }
    }

    pub fn create(conn: &PgConnection, entry: &Self) -> Result<Self, Box<dyn Error>> {
        diesel::insert_into(precipitation_logs::table)
            .values(entry)
            .execute(conn)?;

        Ok(precipitation_logs::table
            .filter(precipitation_logs::id.eq(entry.id))
            .first(conn)?
        )
    }

    pub fn read_all(conn: &PgConnection) -> Result<Vec<Self>, Box<dyn Error>> {
        Ok(precipitation_logs::table
            .filter(precipitation_logs::deleted.eq(false))
            .load::<PrecipitationLog>(conn)?
        )
    }

    pub fn read(conn: &PgConnection, id: Uuid) -> Result<Option<Self>, Box<dyn Error>> {
        let result: Vec<PrecipitationLog> = precipitation_logs::table
            .filter(precipitation_logs::id.eq(id))
            .load::<PrecipitationLog>(conn)?;

        if result.len() == 1 {
            Ok(Some(result[0].clone()))
        } else {
            Ok(None)
        }
    }

    pub fn upsert(conn: &PgConnection, user_entry: &Self) -> Result<Self, Box<dyn Error>> {
        match PrecipitationLog::read(conn, user_entry.id)? {
            Some(db_entry) => {
                let entry = PrecipitationLog::new_for_upsert(&db_entry, user_entry);
                PrecipitationLog::update(conn, &entry)
            }
            None => {
                let entry = PrecipitationLog::new(
                    user_entry.measurement,
                    user_entry.logged_at,
                    PrecipitationType::from_i16(user_entry.ptype),
                    user_entry.notes.to_owned(),
                    user_entry.anomaly,
                );
                PrecipitationLog::create(conn, &entry)
            }
        }
    }

    pub fn update(conn: &PgConnection, entry: &Self) -> Result<Self, Box<dyn Error>> {
        diesel::update(precipitation_logs::table)
            .set(entry)
            .filter(precipitation_logs::id.eq(entry.id))
            .execute(conn)?;

        let result = PrecipitationLog::read(conn, entry.id)?;

        match result {
            Some(e) => Ok(e),
            None => Err(Box::new(diesel::NotFound))
        }
    }

    pub fn soft_delete(conn: &PgConnection, id: Uuid) -> Result<(), Box<dyn Error>> {
        diesel::update(precipitation_logs::table)
            .set(precipitation_logs::deleted.eq(true))
            .filter(precipitation_logs::id.eq(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn delete(conn: &PgConnection, id: Uuid) -> Result<(), Box<dyn Error>> {
        diesel::delete(precipitation_logs::table)
            .filter(precipitation_logs::id.eq(id))
            .execute(conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;

    use dotenv::dotenv;

    use super::*;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    /*
     * These really are integration tests, not unit tests. I've marked them all
     * ignored as they shouldn't be called during the normal test runs, and only
     * are useful for working on the database.
     */

    #[test]
    #[ignore]
    fn create_precipitation_log_entry() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let entry = PrecipitationLog::new(
            5.0,
            Utc::now(),
            PrecipitationType::Liquid,
            Some("This is a note".to_string()),
            true,
        );
        let result = PrecipitationLog::create(&connection, &entry).expect("Failed to create entry");
        assert_eq!(entry.measurement, result.measurement);
        assert_eq!(entry.logged_at.timestamp(), result.logged_at.timestamp());
        assert_eq!(entry.ptype, result.ptype);
        assert_eq!(entry.notes.unwrap_or_default(), result.notes.unwrap_or_default());
        assert_eq!(entry.anomaly, result.anomaly);
    }

    #[test]
    #[ignore]
    fn update_precipitation_log_entry() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let id = Uuid::parse_str("f76f799b-c7be-4553-a48d-8c282df7cc9c").expect("Failed to parse ID");
        let mut entry = PrecipitationLog::read(&connection, id).expect("Failed to fetch entry.").unwrap();
        let expected_measurement = 10.0;
        let expected_notes = Some("I have changed the notes.".to_string());
        entry.measurement = expected_measurement;
        entry.notes = expected_notes.clone();
        let result = PrecipitationLog::update(&connection, &entry).expect("Failed to update entry.");
        assert_eq!(expected_measurement, result.measurement);
        assert_eq!(expected_notes.unwrap(), entry.notes.expect("Result notes unset."));
    }

    #[test]
    #[ignore]
    fn soft_delete_precipitation_log_entry() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let id = Uuid::parse_str("5994b445-a8d8-4918-a1ca-00d09299688f").expect("Failed to parse ID");
        let before_result = PrecipitationLog::read_all(&connection).expect("Failed to read all.");
        PrecipitationLog::soft_delete(&connection, id).expect("Failed to delete entry.");
        let after_result = PrecipitationLog::read_all(&connection).expect("Failed to read all.");
        assert_eq!(before_result.len() - 1, after_result.len());
    }

    #[test]
    #[ignore]
    fn delete_precipitation_log_entry() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let id = Uuid::parse_str("f76f799b-c7be-4553-a48d-8c282df7cc9c").expect("Failed to parse ID");
        PrecipitationLog::delete(&connection, id).expect("Failed to delete entry.");
        let result = PrecipitationLog::read_all(&connection).expect("Failed to read all.");
        assert_eq!(0, result.len());
    }
}
