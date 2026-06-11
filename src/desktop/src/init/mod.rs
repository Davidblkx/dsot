#[cfg(feature = "desktop")]
mod desktop;

#[cfg(not(feature = "desktop"))]
mod mobile;

#[cfg(feature = "desktop")]
pub async fn init_app() {
    desktop::init_desktop().await;
}

#[cfg(not(feature = "desktop"))]
pub async fn init_app() {
    mobile::init_mobile().await;
}

