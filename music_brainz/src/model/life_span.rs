use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LifeSpan {
    pub begin: Option<String>,
    pub end: Option<String>,
    pub ended: Option<bool>
}

impl LifeSpan {
    pub fn has_ended(&self) -> bool {
        self.ended.unwrap_or(false)
    }
}
