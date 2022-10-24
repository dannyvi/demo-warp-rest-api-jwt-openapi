use demo_db::models::user::User;
use hyper::http;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use crate::conf::context::Context;

// GET /accounts/<address>
pub fn get_users(context: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(warp::get())
        .and(warp::path::end())
        .and(context.filter())
        .and_then(handle_get_users)
        .boxed()
}

async fn handle_get_users(ctx: Context) -> Result<impl Reply, Rejection> {

    Ok(
        // warp::reply::with_header(
        warp::reply::json(&User::find_all(&ctx.db_conn()).unwrap()),
        // http::header::CONTENT_TYPE,
        // http::HeaderValue::from_static("application/json"),
        // )
    )
}
