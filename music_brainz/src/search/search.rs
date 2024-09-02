use reqwest::header::USER_AGENT;

use super::SearchQuery;
use crate::error::Result;

/// Get the search URL for a given query.
///
/// # Arguments
///
/// * `query` - The search query.
pub fn get_search_url<T: SearchQuery>(query: &T) -> Result<url::Url> {
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

/// Search for entities based on a search query.
///
/// # Arguments
///
/// * `query` - The search query.
pub async fn execute_search<T: SearchQuery>(query: &T) -> Result<reqwest::Response> {
    let url = query.build_url()?;
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, "music_brainz_rs/0.1.0 (dev@davidpires.pt)")
        .send().await?;

    Ok(res)
}
