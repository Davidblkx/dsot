pub trait SearchQuery {
    fn target(&self) -> &'static str;
    fn query_value(&self) -> &str;
    fn limit(&self) -> u32;
    fn offset(&self) -> u32;
}
