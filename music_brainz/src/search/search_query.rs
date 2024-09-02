pub trait SearchQuery {
    fn target(&self) -> &'static str;
    fn query_value(&self) -> &str;
    fn limit(&self) -> u32;
    fn offset(&self) -> u32;

    fn build_url(&self) -> crate::error::Result<url::Url>;

    fn execute(&self) -> impl std::future::Future<Output = crate::error::Result<serde_json::Value>> + Send;
}
