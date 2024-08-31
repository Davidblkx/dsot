pub struct ArtistsSearchQuery {
    parts: Vec<String>,
}

impl ArtistsSearchQuery {
    pub fn create() -> Self {
        ArtistsSearchQuery {
            parts: Vec::new(),
        }
    }

    pub fn begin_group(&mut self) -> &mut Self {
        self.parts.push("(".to_string());
        self
    }

    pub fn end_group(&mut self) -> &mut Self {
        self.parts.push(")".to_string());
        self
    }

    pub fn and(&mut self) -> &mut Self {
        self.parts.push(" AND ".to_string());
        self
    }

    pub fn or(&mut self) -> &mut Self {
        self.parts.push(" OR ".to_string());
        self
    }

    pub fn not(&mut self) -> &mut Self {
        self.parts.push(" NOT ".to_string());
        self
    }

    pub fn build(&self) -> String {
        self.parts.join("")
    }
}

query_prop_str!(ArtistsSearchQuery, artist);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let query = ArtistsSearchQuery::create();
        assert_eq!(query.parts.len(), 0);
    }

    #[test]
    fn test_begin_group() {
        let mut query = ArtistsSearchQuery::create();
        query.begin_group();
        assert_eq!(query.parts.len(), 1);
        assert_eq!(query.parts[0], "(".to_string());
    }

    #[test]
    fn test_end_group() {
        let mut query = ArtistsSearchQuery::create();
        query.end_group();
        assert_eq!(query.parts.len(), 1);
        assert_eq!(query.parts[0], ")".to_string());
    }

    #[test]
    fn test_and() {
        let mut query = ArtistsSearchQuery::create();
        query.and();
        assert_eq!(query.parts.len(), 1);
        assert_eq!(query.parts[0], " AND ".to_string());
    }

    #[test]
    fn test_or() {
        let mut query = ArtistsSearchQuery::create();
        query.or();
        assert_eq!(query.parts.len(), 1);
        assert_eq!(query.parts[0], " OR ".to_string());
    }

    #[test]
    fn test_not() {
        let mut query = ArtistsSearchQuery::create();
        query.not();
        assert_eq!(query.parts.len(), 1);
        assert_eq!(query.parts[0], " NOT ".to_string());
    }

    #[test]
    fn test_build() {
        let mut query = ArtistsSearchQuery::create();
        query.begin_group().artist("name").and().artist("name2").end_group();
        assert_eq!(query.build(), "(artist:name AND artist:name2)");
    }
}
