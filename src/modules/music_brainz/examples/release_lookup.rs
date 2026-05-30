use music_brainz::entities::Release;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Release::lookup("591f767a-b046-3671-afc7-aaea3804dc49")
        .inc_media()
        .inc_recordings()
        .execute().await;

    match result {
        Ok(r) => {
            println!("{}: {} [{}]", r.id, r.title, r.date.unwrap_or("".to_string()));
            if let Some(status) = r.status {
                println!("Status: {}", status);
            }
            if let Some(country) = r.country {
                println!("Country: {}", country);
            }

            match r.media {
                Some(media) => {
                    for m in media {
                        println!(
                            "Media: {} - [{}]",
                            m.position.unwrap_or(0),
                            m.format.unwrap_or("".to_string())
                        );

                        if let Some(title) = m.title {
                            println!("Title: {}", title);
                        }
                        if let Some(count) = m.track_count {
                            println!("Track Count: {}", count);
                        }

                        if let Some(tracks) = m.tracks {
                            for track in tracks {
                                println!(
                                    "Track: {} - {}",
                                    track.position.unwrap_or(0),
                                    track.title.unwrap_or("".to_string())
                                );
                            }
                        }
                    }
                }
                None => println!("No media information available."),
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
