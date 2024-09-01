use super::SearchQuery;

// https://crates.io/crates/url
pub fn get_search_url(query: impl SearchQuery) -> String {
    format!(
        "https://musicbrainz.org/ws/2/{}?query={}&limit={}&offset={}&fmt=json",
        query.target(),
        query.query_value(),
        query.limit(),
        query.offset()
    )
}
