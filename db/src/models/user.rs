use crate::schema::users::dsl::*;
use crate::{database::DbConn, schema::users};
use chrono::NaiveDateTime;
use diesel::{QueryResult, RunQueryDsl, QueryDsl};
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
    pub fn find_all(conn: &DbConn) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }

    pub fn find_by_id(user_id: &String, conn: &DbConn) -> QueryResult<User> {
        users.find(user_id).get_result::<User>(conn)
    }

    // pub fn insert(new_users: UserDTO, conn: &Connection) -> QueryResult<usize> {
    //     diesel::insert_into(users)
    //         .values(&new_users)
    //         .execute(conn)
    // }

    // pub fn update(i: i32, updated_users: UserDTO, conn: &Connection) -> QueryResult<usize> {
    //     diesel::update(users.find(i))
    //         .set(&updated_users)
    //         .execute(conn)
    // }

    // pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
    //     diesel::delete(users.find(i)).execute(conn)
    // }
}
