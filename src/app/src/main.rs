mod views;
mod widgets;

#[cfg(feature = "desktop")]
mod desktop;

#[cfg(feature = "mobile")]
mod mobile;

#[cfg(feature = "desktop")]
use desktop::init;

#[cfg(feature = "mobile")]
use mobile::init;

#[tokio::main]
async fn main() {
    init().await;
}
