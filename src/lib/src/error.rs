#[derive(Debug, thiserror::Error)]
pub enum DsotError {}

pub type Result<T> = std::result::Result<T, DsotError>;
