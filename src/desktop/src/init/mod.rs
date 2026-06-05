#[cfg(feature = "desktop")]
mod desktop;

#[cfg(feature = "mobile")]
mod mobile;

#[cfg(feature = "desktop")]
pub fn init_app() {
    desktop::init_desktop();
}

#[cfg(feature = "mobile")]
pub fn init_app() {
    mobile::init_mobile();
}
