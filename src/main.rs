use crate::server::server;

mod context;
mod server;

#[tokio::main]
async fn main() {
    server().await.expect("hyper error");
}
