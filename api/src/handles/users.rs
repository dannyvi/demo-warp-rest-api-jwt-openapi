use demo_db::models::user::User;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use crate::{context::{context::{Context, AuthInfo}}, errors::ApiError};

// GET /accounts/<address>
pub fn get_users(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(warp::get())
        .and(warp::path::end())
        // .and(ctx.filter_user_info())
        .and(warp::header("authorization")
        .map(|token: String| {token
        })
        .or(warp::any().map(|| {
            "".to_owned()
        }))
        .unify())
        .and(ctx.filter())
        .map(|token, ctx: Context| (ctx.filter_user_info(token), ctx))
        .untuple_one()
        .and_then(handle_get_users)
        .boxed()
}

async fn handle_get_users(info: Result<AuthInfo, ApiError> , ctx: Context) -> Result<impl Reply, Rejection> {
    println!("the auth info {:?}", info);
    Ok(warp::reply::json(&User::find_all(&ctx.db_conn()).unwrap()))
}
