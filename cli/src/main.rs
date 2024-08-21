use dsot_core::setup::{SetupConfig, LibConfig};

fn main() {
    // TODO: Add proper error logging

    let dsot = dsot_core::setup(SetupConfig {
        local_data: std::path::PathBuf::from("./"),
        lib: LibConfig {
            name: "library".to_string(),
        },
    }).expect("Failed to setup DSOT");

    let mut lib_conn = dsot.db_manager
        .get("library")
        .expect("Failed to get library db");

    dsot_core::entities::album::create_new_album(
        &mut lib_conn.connection,
        "Test Album",
        None,
        None
    ).expect("Failed to create album");

    let albums = dsot_core::entities::album::query::list_albums(&mut lib_conn.connection)
        .expect("Failed to list albums");

    for album in albums {
        println!("Album: {}", album.name);
    }
}
