use crate::conf::config::CONFIG;
use std::{net::SocketAddr, convert::Infallible};

use hyper::Server;
use listenfd::ListenFd;
use warp::{Filter };

pub async fn server() -> Result<(), hyper::Error> {
    let server_addr = CONFIG
        .server
        .as_str()
        .parse::<SocketAddr>()
        .expect("Unable to parse socket address");
    let routes = warp::any().map(|| "Hello, World!");
    
    
    let svc = warp::service(
        routes
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
