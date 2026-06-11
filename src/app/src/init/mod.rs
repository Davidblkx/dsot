#[cfg(feature = "desktop")]
mod desktop;

#[cfg(feature = "mobile")]
mod mobile;

#[cfg(feature = "desktop")]
pub async fn init_app() {
    desktop::init_desktop().await;
}

#[cfg(feature = "mobile")]
pub async fn init_app() {
    mobile::init_mobile().await;
}
