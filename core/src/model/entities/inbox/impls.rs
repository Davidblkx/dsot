use super::Inbox;
impl Inbox {
    pub fn new() -> Self {
        Inbox {
            id: uuid::Uuid::now_v7(),
            user_id: uuid::Uuid::nil(),
            title: None,
            artist: None,
            album: None,
            file: None,
            extra_info: None,
        }
    }
}
