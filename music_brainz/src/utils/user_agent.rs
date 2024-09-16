use std::sync::OnceLock;

use crate::error::{MusicBrainzError, Result};

static USER_AGENT: OnceLock<String> = OnceLock::new();

///
/// Initialize the user agent for the MusicBrainz API.
/// Create a user agent string in the format of `app_name/version (email)`.
/// It must be called before making any requests to the MusicBrainz API.
/// see: https://musicbrainz.org/doc/XML_Web_Service/Rate_Limiting#User-Agent
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `version` - The version of the application.
/// * `email` - The email of the application's maintainer.
///
/// # Errors
///
/// Returns an error if the user agent has already been initialized.
///
pub fn init_user_agent(app_name: &str, version: &str, email: &str) -> Result<()> {
    let user_agent = format!("{}/{} ({})", app_name, version, email);
    USER_AGENT.set(user_agent).map_err(|_| MusicBrainzError::UserAgentAlreadyInitialized)?;
    Ok(())
}

pub fn get_user_agent() -> Result<String> {
    USER_AGENT.get().cloned().ok_or(MusicBrainzError::UserAgentNotInitialized)
}
