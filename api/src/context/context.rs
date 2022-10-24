use std::convert::Infallible;

use demo_db::{database::Connection, Database};
use warp::{Filter, Rejection};

use crate::errors::ApiError;

use super::jwt::{Claims, Jwt};

#[derive(Clone)]
pub struct Context {
    db: Database,
    jwt: Jwt,
}

impl Context {
    pub fn new(database_url: String, secret: String, expire: i64) -> Self {
        let db = Database::new(database_url);
        let jwt = Jwt::new(secret, expire);
        Self { db, jwt }
    }

    pub fn db_conn(&self) -> Connection {
        self.db.conn()
    }

    pub fn filter(self) -> impl Filter<Extract = (Context,), Error = Infallible> + Clone {
        warp::any().map(move || self.clone())
    }

    // pub fn expand(&self,) -> impl Filter<Extract = (AuthInfo, Connection), Error = Infallible> + Clone {
    //     // warp::any().map(move || self.clone())
    //     warp::header("authorization")
    //         .map(|token: String| token)
    //         .or(warp::any().map(|| "".to_owned()))
    //         .unify()
    //         .map(move |token| return (self.filter_user_info(token), self.db_conn()))
    // }

    pub fn jwt_claim(&self, id: String, permissions: Vec<String>) -> anyhow::Result<Claims> {
        self.jwt.new_claim(id, permissions)
    }

    pub fn jwt_encode(&self, claims: Claims) -> Result<String, ApiError> {
        self.jwt.encode(claims)
    }

    pub fn _jwt_decode(&self, token: String) -> Result<Claims, ApiError> {
        // println!("token {:?}", token);
        self.jwt.decode(token.as_str())
    }

    pub fn filter_user_info(&self, token: String) -> Result<AuthInfo, ApiError> {
        if token == "".to_owned() {
            Ok(AuthInfo {
                user_id: "0".to_owned(),
                permissions: vec!["ROLE_ANY".to_owned()],
            })
        } else if !token.starts_with("Bearer ") {
            Err(ApiError::Unauthorized("e".to_string()))
        } else {
            let trimmed = token.trim_start_matches("Bearer ").to_owned();
            self._jwt_decode(trimmed)
                .map(|claims| AuthInfo::from(claims))
            // match self._jwt_decode(trimmed) {
            //     Ok(claims) => Ok(AuthInfo::from(claims)),
            //     Err(e) => Err(e)
            // }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AuthInfo {
    user_id: String,
    permissions: Vec<String>,
}

impl From<Claims> for AuthInfo {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.id,
            permissions: claims.permissions,
        }
    }
}
