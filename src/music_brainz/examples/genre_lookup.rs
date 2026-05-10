use music_brainz::entities::Genre;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Genre::lookup("41569e0d-9b3f-4ad0-8ddc-85da7cf66e5b")
        .execute().await;

    match result {
        Ok(a) => {
            println!("{}: {}", a.id, a.name);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
