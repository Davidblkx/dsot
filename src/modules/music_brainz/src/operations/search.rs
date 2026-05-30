use reqwest::header::USER_AGENT;
use crate::error::Result;

/// Trait for search queries, used to build search queries for the MusicBrainz API.
pub trait SearchQuery {
    fn target(&self) -> &'static str;
    fn query_value(&self) -> &str;
    fn limit(&self) -> u32;
    fn offset(&self) -> u32;
}

/// Get the search URL for a given query.
pub fn build_url<T: SearchQuery>(query: &T) -> Result<url::Url> {
    let query_value = urlencoding::encode(query.query_value());

    let url_str = format!(
        "https://musicbrainz.org/ws/2/{}?query={}&limit={}&offset={}&fmt=json",
        query.target(),
        query_value,
        query.limit(),
        query.offset()
    );

    let url = url::Url::parse(&url_str)?;
    Ok(url)
}

pub async fn execute_search<T: SearchQuery>(query: &T) -> Result<String> {
    log::trace!("Executing search query: {:?}", query.target());
    let user_agent = crate::utils::user_agent::get_user_agent()?;
    log::trace!("User agent: {}", user_agent);
    let url = build_url(query)?;
    log::debug!("Search URL: {}", url);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send().await?.text().await?;
    log::trace!("Search completed successfully");

    Ok(res)
}
