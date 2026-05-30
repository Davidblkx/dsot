use reqwest::header::USER_AGENT;
use crate::error::Result;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Genre {
    pub name: String,
    pub count: Option<u32>,
    pub disambiguation: Option<String>,
    pub id: String,
}

mb_lookup!{Genre{
    target = "genre",
    inc =
}}

impl Genre {
    pub fn list() -> GenreListRequest {
        GenreListRequest::default()
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct GenreList {
    pub genres: Vec<Genre>,
    #[serde(alias = "genre-offset")]
    pub offset: u32,
    #[serde(alias = "genre-count")]
    pub count: u32,
}

#[derive(Clone, Debug)]
pub struct GenreListRequest {
    pub offset: Option<u32>,
    pub count: Option<u32>,
}

impl Default for GenreListRequest {
    fn default() -> Self {
        Self {
            offset: None,
            count: None,
        }
    }
}

impl GenreListRequest {
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.count = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn build_url(&self) -> Result<url::Url> {
        let base_url = "https://musicbrainz.org/ws/2/genre/all?fmt=json";

        let url_str = match (self.offset, self.count) {
            (Some(offset), Some(count)) => format!("{}&offset={}&limit={}", base_url, offset, count),
            (Some(offset), None) => format!("{}&offset={}", base_url, offset),
            (None, Some(count)) => format!("{}&limit={}", base_url, count),
            _ => base_url.to_string(),
        };

        let url = url::Url::parse(&url_str)?;
        Ok(url)
    }

    pub async fn execute(&self) -> Result<GenreList> {
        log::trace!("Listing genres");
        let user_agent = crate::utils::user_agent::get_user_agent()?;
        log::trace!("User agent: {}", user_agent);
        let url = self.build_url()?;
        log::debug!("List genre URL: {}", url);
        let client = reqwest::Client::new();
        let json_src = client
            .get(url)
            .header(USER_AGENT, user_agent)
            .send().await?.text().await?;
        log::trace!("List request completed successfully");
        let res: GenreList = crate::utils::safe_parse_json::parse(json_src)?;
        Ok(res)
    }
}
