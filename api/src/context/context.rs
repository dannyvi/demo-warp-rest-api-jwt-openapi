use std::convert::Infallible;

use demo_db::{database::Connection, Database};
use warp::{reject, Filter, Rejection};

use crate::errors::ApiError;

use super::{
    auth::AuthUser,
    jwt::{Claims, Jwt},
};

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

    pub fn _db_conn(&self) -> Connection {
        self.db.conn()
    }

    pub fn filter(self) -> impl Filter<Extract = (Context,), Error = Infallible> + Clone {
        warp::any().map(move || self.clone())
    }

    pub fn expand(
        self,
    ) -> impl Filter<Extract = ((AuthUser, Connection),), Error = Rejection> + Clone {
        warp::header("authorization")
            .map(|token: String| token)
            .or(warp::any().map(|| "".to_owned()))
            .unify()
            .and(self.filter())
            .and_then(move |token, ctx: Context| async move {
                let res = ctx._filter_user_info(token);
                if res.is_err() {
                    Err(reject::custom(res.unwrap_err()))
                } else {
                    Ok((res.unwrap(), ctx._db_conn()))
                }
            })
    }

    pub fn db(self) -> impl Filter<Extract = (Connection,), Error = Rejection> + Clone {
        self.filter().and_then(move |ctx: Context| async move {
            Ok::<Connection, Rejection>(ctx._db_conn())
        })
    }

    pub fn auth(self) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
        warp::header("authorization")
            .map(|token: String| token)
            .or(warp::any().map(|| "".to_owned()))
            .unify()
            .and(self.filter())
            .and_then(move |token, ctx: Context| async move {
                let res = ctx._filter_user_info(token);
                if res.is_err() {
                    Err(reject::custom(res.unwrap_err()))
                } else {
                    Ok(res.unwrap())
                }
            })
    }

    pub fn _jwt_claim(&self, id: String, permissions: Vec<String>) -> anyhow::Result<Claims> {
        self.jwt.new_claim(id, permissions)
    }

    pub fn _jwt_encode(&self, claims: Claims) -> Result<String, ApiError> {
        self.jwt.encode(claims)
    }

    pub fn _jwt_decode(&self, token: String) -> Result<Claims, ApiError> {
        // println!("token {:?}", token);
        self.jwt.decode(token.as_str())
    }

    pub fn _filter_user_info(&self, token: String) -> Result<AuthUser, ApiError> {
        if token == "".to_owned() {
            Ok(AuthUser::default())
        } else if !token.starts_with("Bearer ") {
            Err(ApiError::Unauthorized(
                "Authorization should begin with Bearer.".to_string(),
            ))
        } else {
            let trimmed = token.trim_start_matches("Bearer ").to_owned();
            self._jwt_decode(trimmed)
                .map(|claims| AuthUser::from(claims))
        }
    }
}
