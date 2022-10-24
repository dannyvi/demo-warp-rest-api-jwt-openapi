use crate::server::server;

mod conf;
mod server;
mod handles;
mod index;
mod errors;

#[tokio::main]
async fn main() {
    server().await.expect("hyper error");
}