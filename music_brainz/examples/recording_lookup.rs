use music_brainz::entities::Recording;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Recording::lookup("6dc462de-a12b-4cea-878b-52b031be8257")
        .inc_artists()
        .inc_genres()
        .execute().await;

    match result {
        Ok(a) => {
            println!("{}: {} ({})", a.id, a.title, a.first_release_date.unwrap_or_default());

            if let Some(artists) = a.artists {
                for artist in artists {
                    println!("Artist: {}", artist.name);
                }
            }

            if let Some(genres) = a.genres {
                for genre in genres {
                    println!("Genre: {}", genre.name);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
