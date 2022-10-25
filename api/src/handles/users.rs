use std::convert::Infallible;

use demo_db::{models::user::User, database::DbConn};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use crate::{context::{context::Context, auth::AuthUser}, errors::ApiError};



pub fn routes(context: Context) ->  impl Filter<Extract = impl Reply, Error = Rejection> + Clone { 
    list(context.clone()).or(retrieve(context.clone()))
}

pub fn list(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(warp::get())
        .and(warp::path::end())
        .and(ctx.clone().auth())
        .and(ctx.clone().db())
        .and_then(handle_list)
        .boxed()
}

async fn handle_list(_: AuthUser, conn: DbConn) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&User::find_all(&conn).unwrap()))
}


pub fn retrieve(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path!("users" / String)
        .and(warp::get())
        .and(warp::path::end())
        .and(ctx.clone().db())
        .and_then(handle_retrieve)
        .boxed()
}

async fn handle_retrieve(id: String, conn: DbConn) -> Result<impl Reply, Rejection> {
    match User::find_by_id(&id, &conn) {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(err) => Err(warp::reject::custom(ApiError::from(err))),
    }
}

