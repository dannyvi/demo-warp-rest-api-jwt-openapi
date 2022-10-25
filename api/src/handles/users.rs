use demo_db::{models::user::User, database::Connection};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use crate::{context::{context::Context, auth::AuthUser}};

pub fn get_users(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(warp::get())
        .and(warp::path::end())
        // .and(ctx.expand().untuple_one())
        .and(ctx.clone().auth())
        .and(ctx.clone().db())
        .and_then(handle_get_users)
        .boxed()
}

async fn handle_get_users(auth: AuthUser, conn: Connection) -> Result<impl Reply, Rejection> {
    
    println!("the auth info {:?}", auth);
    Ok(warp::reply::json(&User::find_all(&conn).unwrap()))
}
