use std::convert::Infallible;

use hyper::header;
use warp::{Filter, Reply};

use crate::{handles::users::get_users, conf::context::Context, errors::handle_rejection};



pub fn routes(context: Context) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    get_users(context.clone())
       
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["POST", "GET"])
                .allow_headers(vec![header::CONTENT_TYPE]),
        )
        .recover(handle_rejection)
        // .with(log::logger())
}