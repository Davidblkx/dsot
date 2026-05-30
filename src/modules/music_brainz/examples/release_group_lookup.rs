use music_brainz::entities::ReleaseGroup;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = ReleaseGroup::lookup("51c88244-97b3-4ae5-9ffa-3a4530bb6343")
        .inc_artists()
        .inc_tags()
        .inc_annotation()
        .inc_genres()
        .execute().await;

    match result {
        Ok(a) => {
            println!("{}: {}({})", a.id, a.title, a.primary_type.unwrap_or_default());
            if let Some(aliases) = a.artists {
                for alias in aliases {
                    println!("Artist: {}", alias.name);
                }
            }

            if let Some(tags) = a.tags {
                for tag in tags {
                    println!("Tag: {} ({})", tag.name, tag.count);
                }
            }

            if let Some(annotation) = a.annotation {
                println!("Annotation: {}", annotation);
            }

            if let Some(genres) = a.genres {
                for genre in genres {
                    println!("Genre: {}", genre.name);
                }
            }

            if let Some(aliases) = a.secondary_types {
                for alias in aliases {
                    println!("Secondary Type: {}", alias);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
