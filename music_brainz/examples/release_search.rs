use music_brainz::entities::Release;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let release = Release::search()
        .release("Bone Machine")
        .limit(5)
        .build();

    let result = release.execute().await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);
            for item in a.releases {
                let first_artist = match item.artists {
                    Some(artists) => artists.first().map(|artist| artist.name.clone()).unwrap_or("".to_string()),
                    None => "".to_string(),
                };

                println!(
                    "{}: {}({})",
                    item.id,
                    item.title,
                    first_artist
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
