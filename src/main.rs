use crate::server::server;

mod conf;
mod server;

#[tokio::main]
async fn main() {
    server().await.expect("hyper error");
}
