use crate::{
    context::{config::CONFIG, context::Context},
    index::routes,
};
use std::{convert::Infallible, net::SocketAddr};

use hyper::Server;
use listenfd::ListenFd;

pub async fn server() -> Result<(), hyper::Error> {
    let server_addr = CONFIG
        .clone()
        .server
        .as_str()
        .parse::<SocketAddr>()
        .expect("Unable to parse socket address");
    // let routes = warp::any().map(|| "Hello, World!");

    let context = Context::new(CONFIG.clone().database_url, CONFIG.clone().secret, CONFIG.clone().expire);

    let svc = warp::service(routes(context));

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
