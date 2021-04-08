use crate::schema::users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;
use std::error::Error;
use chrono::prelude::*;

#[table_name = "users"]
#[derive(AsChangeset, Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl User {
    pub fn create(conn: &PgConnection, user: User) -> Result<User, Box<dyn Error>> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)?;

        Ok(users::table.filter(users::id.eq(user.id)).first(conn)?)
    }

    pub fn read_all(conn: &PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
        Ok(users::table.load::<User>(conn)?)
    }

    pub fn read(conn: &PgConnection, id: Uuid) -> Result<Vec<User>, Box<dyn Error>> {
        Ok(users::table.filter(users::id.eq(id)).load(conn)?)
    }

    pub fn update(conn: &PgConnection, id: Uuid, update: UserUpdateSet) -> Result<(), Box<dyn Error>> {
        diesel::update(users::table).set(&update).filter(users::id.eq(id)).execute(conn)?;

        Ok(())
    }

    pub fn delete(conn: &PgConnection, id: Uuid) -> Result<(), Box<dyn Error>> {
        diesel::delete(users::table).filter(users::id.eq(id)).execute(conn)?;

        Ok(())
    }
}

#[table_name = "users"]
#[derive(AsChangeset)]
pub struct UserUpdateSet {
    pub name: Option<String>,
    pub password: Option<String>,
    pub enabled: Option<bool>,
    pub modified_at: DateTime<Utc>
}

impl UserUpdateSet {
    pub fn new(name: Option<String>, password: Option<String>, enabled: Option<bool>) -> UserUpdateSet {
        return UserUpdateSet {
            name,
            password,
            enabled,
            modified_at: Utc::now()
        };
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserJson {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl UserJson {
    pub fn transform(user: &User) -> UserJson {
        UserJson {
            id: user.id.to_owned(),
            name: user.name.to_owned(),
            enabled: user.enabled,
            created_at: user.created_at.clone(),
            modified_at: user.modified_at.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::{DEFAULT_COST, hash};
    use dotenv::dotenv;
    use std::env;
    use std::sync::Once;

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
    fn test_create_user() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let hash = hash("hunter2", DEFAULT_COST).unwrap();
        let id = Uuid::parse_str("88280065-d1da-4255-8e29-0a09da2da88a").unwrap();
        let user = User {
            id,
            name: "testuser".to_string(),
            password: hash,
            enabled: true,
            created_at: Utc::now(),
            modified_at: Utc::now()
        };
        User::create(&connection, user).expect("Failed to create user.");
        let expected_name = vec!["testuser".to_string()];
        let names = users::table.select(users::name).filter(users::id.eq(id)).load(&connection);
        assert_eq!(Ok(expected_name), names);
    }

    #[test]
    #[ignore]
    fn test_update_user() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let id = Uuid::parse_str("88280065-d1da-4255-8e29-0a09da2da88a").unwrap();
        let update_user = UserUpdateSet::new(
            Some("testuser2".to_string()),
            None,
            None
        );
        User::update(&connection, id, update_user).unwrap();
        let result = User::read(&connection, id).unwrap();
        let user = result.get(0).unwrap();
        let expected_name = "testuser2".to_string();
        assert_eq!(expected_name, user.name);
    }

    #[test]
    #[ignore]
    fn test_delete_user() {
        initialize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let id = Uuid::parse_str("88280065-d1da-4255-8e29-0a09da2da88a").unwrap();
        User::delete(&connection, id).unwrap();
        let result = User::read_all(&connection).unwrap();
        assert_eq!(0, result.len());
    }
}
