#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalLibraryV0 {
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalLibraryV1 {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<LocalLibraryV0> for LocalLibraryV1 {
    fn from(model: LocalLibraryV0) -> Self {
        Self {
            id: model.id,
            name: "".to_string(),
        }
    }
}

crate::dsot_storage_declare_model!(LocalLibrary {
    0: LocalLibraryV0,
    1: LocalLibraryV1
});
