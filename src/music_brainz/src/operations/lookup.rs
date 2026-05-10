use reqwest::header::USER_AGENT;
use crate::error::Result;

pub trait EntityLookup {
    fn target(&self) -> &'static str;
    fn mbid(&self) -> &str;
    fn includes(&self) -> &Vec<String>;
}

/// Get the lookup URL for a given entity.
pub fn build_url<T: EntityLookup>(lookup: &T) -> Result<url::Url> {
    let includes = lookup.includes().join("+");
    let url_str = format!(
        "https://musicbrainz.org/ws/2/{}/{}?inc={}&fmt=json",
        lookup.target(),
        lookup.mbid(),
        includes
    );

    let url = url::Url::parse(&url_str)?;
    Ok(url)
}

pub async fn execute_lookup<T: EntityLookup>(lookup: &T) -> Result<String> {
    log::trace!("Executing lookup query: {:?}", lookup.target());
    let user_agent = crate::utils::user_agent::get_user_agent()?;
    log::trace!("User agent: {}", user_agent);
    let url = build_url(lookup)?;
    log::debug!("Lookup URL: {}", url);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send().await?.text().await?;
    log::trace!("Lookup completed successfully");

    Ok(res)
}
