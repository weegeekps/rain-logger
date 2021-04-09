use std::error::Error;

use chrono::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::user::User;
use crate::schema::api_tokens;

#[derive(Identifiable, Associations, AsChangeset, Queryable, Insertable)]
#[belongs_to(User)]
#[table_name = "api_tokens"]
pub struct ApiToken {
    pub id: Uuid,
    pub force_invalid: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub user_id: Uuid,
}

impl ApiToken {
    pub fn create(conn: &PgConnection, token: ApiToken) -> Result<ApiToken, Box<dyn Error>> {
        diesel::insert_into(api_tokens::table)
            .values(&token)
            .execute(conn)?;

        Ok(api_tokens::table.filter(api_tokens::id.eq(token.id)).first(conn)?)
    }

    pub fn create_for_user(conn: &PgConnection, user: User) -> Result<ApiToken, Box<dyn Error>> {
        let token = ApiToken {
            id: Uuid::new_v4(),
            force_invalid: false,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            user_id: user.id
        };
        ApiToken::create(conn, token)
    }

    pub fn invalidate(conn: &PgConnection, id: Uuid) -> Result<(), Box<dyn Error>> {
        let update_set = ApiTokenUpdateSet {
            force_invalid: true,
            modified_at: Utc::now(),
        };

        diesel::update(api_tokens::table)
            .set(&update_set)
            .filter(api_tokens::id.eq(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn invalidate_all_for_user(conn: &PgConnection, user: &User) -> Result<(), Box<dyn Error>> {
        let update_set = ApiTokenUpdateSet {
            force_invalid: true,
            modified_at: Utc::now(),
        };

        diesel::update(api_tokens::table)
            .set(&update_set)
            .filter(api_tokens::user_id.eq(user.id))
            .execute(conn)?;

        Ok(())
    }
}

#[derive(AsChangeset)]
#[table_name = "api_tokens"]
pub struct ApiTokenUpdateSet {
    pub force_invalid: bool,
    pub modified_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;

    use dotenv::dotenv;

    use crate::models::user::User;

    use super::*;

    static INIT: Once = Once::new();

    fn initalize() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    /*
     * These really are integration tests, not unit tests. I've marked them all
     * ignored as they shouldn't be called during the normal test runs, and only
     * are useful for working on the database.
     *
     * Run crate::models::user::User::tests::test_create_user() before running these tests!
     */

    #[test]
    #[ignore]
    fn test_create_api_token() {
        initalize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL msut be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let id = Uuid::new_v4();
        let token = ApiToken {
            id,
            force_invalid: false,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            user_id: Uuid::parse_str("88280065-d1da-4255-8e29-0a09da2da88a").unwrap(),
        };
        ApiToken::create(&connection, token).expect("Failed to create api token.");
        let result = api_tokens::table.find(id).first::<ApiToken>(&connection);
        assert!(result.is_ok())
    }

    #[test]
    #[ignore]
    fn test_invalidate_api_token() {
        initalize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL msut be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

        // YOU MUST UPDATE ID WITH A REAL ID BEFORE TESTING!
        let id = Uuid::parse_str("1de3f9a9-4e61-4415-80a1-2f65275f0a3e").unwrap();

        ApiToken::invalidate(&connection, id).unwrap();
        let result = api_tokens::table.find(id).first::<ApiToken>(&connection);
        let token = result.unwrap();
        assert_eq!(true, token.force_invalid);
    }

    #[test]
    #[ignore]
    fn test_invalidate_all_api_tokens_for_user() {
        initalize();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL msut be set.");
        let connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
        let user_id = Uuid::parse_str("88280065-d1da-4255-8e29-0a09da2da88a").unwrap();
        let user_result = User::read(&connection, user_id).unwrap();
        let user = user_result.get(0).unwrap();
        ApiToken::invalidate_all_for_user(&connection, user).unwrap();
        let result = api_tokens::table.filter(api_tokens::user_id.eq(user_id)).load::<ApiToken>(&connection).unwrap();
        for t in result.into_iter() {
            assert_eq!(true, t.force_invalid);
        }
    }
}
