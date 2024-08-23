use dsot_core::setup::{SetupConfig, LibConfig};

fn main() {
    env_logger::init();

    let dsot = dsot_core::setup(SetupConfig {
        local_data: std::path::PathBuf::from("./target/test_data"),
        lib: LibConfig {
            name: "library".to_string(),
        },
    }).expect("Failed to setup DSOT");

    let mut lib_conn = dsot.connect_library()
        .expect("Failed to get library db");

    let dsotm_id = dsot_core::entities::album::create_new_album(
        &mut lib_conn.connection,
        "Dark Side of the Moon",
        None,
        Some(vec!["DSOTM".to_string(), "DSOTM 1973".to_string()]),
    ).expect("Failed to create album");

    let animals_id = dsot_core::entities::album::create_new_album(
        &mut lib_conn.connection,
        "Animals",
        None,
        Some(vec!["Animals 1977".to_string()]),
    ).expect("Failed to create album");

    let pink_floyd_id = dsot_core::entities::artist::create_new_artist(
        &mut lib_conn.connection,
        "Pink Floyd",
        None,
        Some(vec!["PF".to_string(), "The Pink Floyd".to_string()]),
    ).expect("Failed to create artist");

    dsot_core::entities::rel::ArtistAlbumRelation::new(&pink_floyd_id, &dsotm_id)
        .create_if_new(&mut lib_conn.connection)
        .expect("Failed to create relation");

    dsot_core::entities::rel::ArtistAlbumRelation::new(&pink_floyd_id, &animals_id)
        .create_if_new(&mut lib_conn.connection)
        .expect("Failed to create relation");

    let albums = dsot_core::entities::album::query::list_albums(&mut lib_conn.connection)
        .expect("Failed to list albums");

    for album in albums {
        println!("Album: {}", album.name);
    }
}
