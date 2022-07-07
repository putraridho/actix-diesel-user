use diesel::{prelude::*, OptionalExtension, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn get_some(conn: &PgConnection) -> Result<Vec<User>, DbError> {
        use crate::schema::users::dsl::*;

        let results = users.limit(10).load(conn)?;

        Ok(results)
    }

    pub fn get_by_id(uuid: Uuid, conn: &PgConnection) -> Result<Option<User>, DbError> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(id.eq(uuid.to_string()))
            .first(conn)
            .optional()
            .expect("User not found");

        Ok(user)
    }

    pub fn create_new(new_user: NewUser, conn: &PgConnection) -> Result<User, DbError> {
        use crate::schema::users::dsl::*;

        let new_user = User {
            id: Uuid::new_v4().to_string(),
            name: new_user.name.to_string(),
        };

        diesel::insert_into(users).values(&new_user).execute(conn)?;

        Ok(new_user)
    }

    pub fn update_by_id(
        uuid: Uuid,
        update_user: UpdateUser,
        conn: &PgConnection,
    ) -> Result<Option<User>, DbError> {
        use crate::schema::users::dsl::*;

        let user = diesel::update(users.find(uuid.to_string()))
            .set(update_user)
            .get_result(conn)
            .optional()
            .expect("User not found");

        Ok(user)
    }

    pub fn remove_by_id(uuid: Uuid, conn: &PgConnection) -> Result<bool, DbError> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(uuid.to_string()))).execute(conn)?;

        Ok(true)
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
}
