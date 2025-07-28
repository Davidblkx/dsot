use super::User;
impl User {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            name: name.into(),
        }
    }
}
