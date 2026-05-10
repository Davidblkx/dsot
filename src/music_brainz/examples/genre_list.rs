use music_brainz::entities::Genre;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Genre::list()
        .offset(20)
        .limit(5)
        .execute().await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);
            for item in a.genres {
                println!(
                    "{}: {})",
                    item.id,
                    item.name
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
