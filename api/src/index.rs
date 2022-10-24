use std::convert::Infallible;

use hyper::header;
use warp::{Filter, Reply};

use crate::{
    context::context::Context,
    errors::handle_rejection,
    handles::{health::health_check_route, users::get_users, token::create_token},
};

pub fn routes(context: Context) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    health_check_route()
        .or(get_users(context.clone()))
        .or(create_token(context.clone()))
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["POST", "GET"])
                .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
        .recover(handle_rejection)
    // .with(log::logger())
}
