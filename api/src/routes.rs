use warp::{Filter, Rejection, Reply};

use crate::{
    context::context::Context,
    handles::{health::health_check_route, token::create_token, users},
};

pub fn routes(context: Context) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check_route()
        .or(users::routes(context.clone()))
        .or(create_token(context.clone()))
}
