use crate::{
    context::{config::CONFIG, context::Context},
    routes::routes, errors::handle_rejection,
};
use std::{convert::Infallible, net::SocketAddr};

use hyper::{Server, header};
use listenfd::ListenFd;
use warp::Filter;

pub async fn server() -> Result<(), hyper::Error> {
    let server_addr = &CONFIG
        .server
        .as_str()
        .parse::<SocketAddr>()
        .expect("Unable to parse socket address");
    // let routes = warp::any().map(|| "Hello, World!");

    let context = Context::new(&CONFIG.database_url, &CONFIG.secret, &CONFIG.expire);
    dotenvy::dotenv().ok();
    env_logger::init();
    let log = warp::log("jojo::api");

    let svc = warp::service(
        routes(context)
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(vec!["POST", "GET"])
                    .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]),
            )
            .recover(handle_rejection)
            .with(log),
    );

    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();

    let server = if let Ok(Some(l)) = listenfd.take_tcp_listener(0) {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&server_addr)
    };

    server.serve(make_svc).await
}
