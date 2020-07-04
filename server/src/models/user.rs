use crate::schema::users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;
use std::error::Error;

#[table_name = "users"]
#[derive(AsChangeset, Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub enabled: bool,
}

impl User {
    pub fn create(user: User, conn: &PgConnection) -> Result<User, Box<dyn Error>> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)?;

        Ok(users::table.filter(users::id.eq(user.id)).first(conn)?)
    }

    pub fn read(conn: &PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
        Ok(users::table.load::<User>(conn)?)
    }

    pub fn update(user: User, conn: &PgConnection) -> Result<(), Box<dyn Error>> {
        diesel::update(users::table.find(user.id)).set(&user).execute(conn)?;

        Ok(())
    }

    pub fn delete(id: Uuid, conn: &PgConnection) -> Result<(), Box<dyn Error>> {
        diesel::delete(users::table.find(id)).execute(conn)?;

        Ok(())
    }
}
