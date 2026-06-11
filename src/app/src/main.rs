mod init;
mod layout;
mod routes;
mod views;
mod widgets;

#[tokio::main]
async fn main() {
    init::init_app().await;
}
