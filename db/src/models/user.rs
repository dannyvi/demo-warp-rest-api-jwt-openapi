use crate::schema::users::dsl::*;
use crate::{database::Connection, schema::users};
use chrono::NaiveDateTime;
use diesel::{QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub mobile: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub mobile: String,
}

impl User {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }
}
