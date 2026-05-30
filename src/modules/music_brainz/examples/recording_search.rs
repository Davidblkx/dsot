use music_brainz::entities::Recording;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Recording::search()
        .artist("Tom Waits")
        .recording("Come On Up to the House")
        .limit(5)
        .build()
        .execute().await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);
            for item in a.recordings {
                println!("--- Recording ---");
                println!(
                    "{}: {} ({}))",
                    item.id,
                    item.title,
                    item.first_release_date.unwrap_or_default()
                );

                if let Some(artists) = item.artists {
                    for artist in artists {
                        println!("Artist: {}", artist.name);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
