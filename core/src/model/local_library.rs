/// Represents a local available library.
///
/// A library is a collection of artitsts, albums, and tracks.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalLibraryV0 {
    /// The unique identifier of the library.
    pub id: uuid::Uuid,
    /// The name of the library.
    pub name: String,
    /// The path where the library device is mounted.
    /// This is the root path, in windows is the drive letter, in linux is the mount point.
    pub mount_point: String,
    /// The relative path where the library database is stored. Including the file name.
    pub db_path: String,
}

crate::dsot_storage_declare_model!(LocalLibrary {
    0: LocalLibraryV0
});

crate::dsot_storage_use_id_uuid!(LocalLibrary, "local_library");
