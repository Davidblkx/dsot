pub enum Entity {
    Artist,
}

// TODO: Use some kind of macro to generate this code
impl Entity {
    pub fn to_id(&self) -> u32 {
        match self {
            Entity::Artist => 1,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Entity::Artist => "artist",
        }
    }

    pub fn to_table_name(&self) -> &'static str {
        match self {
            Entity::Artist => "artists",
        }
    }

    pub fn from_id(id: u32) -> Option<Entity> {
        match id {
            1 => Some(Entity::Artist),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Entity> {
        match s {
            "artist" => Some(Entity::Artist),
            _ => None,
        }
    }
}
