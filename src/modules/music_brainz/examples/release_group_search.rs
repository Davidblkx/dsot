use music_brainz::entities::ReleaseGroup;
use music_brainz::model::release_group::ReleaseGroupType;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = ReleaseGroup::search()
        .releasegroup("Weezer")
        .limit(5)
        .build()
        .execute().await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);
            for item in a.release_groups {
                match item.primary_type {
                    Some(ReleaseGroupType::Album) => {
                        let artist_name = match item.main_artist() {
                            Some(artist) => artist.name.clone(),
                            None => "Unknown".to_string(),
                        };

                        println!(
                            "{}: {}({}) by {}",
                            item.id,
                            item.title,
                            item.disambiguation.unwrap_or_default(),
                            artist_name
                        );
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
