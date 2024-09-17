use music_brainz::entities::Artist;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Artist::lookup("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .inc_aliases()
        .inc_tags()
        .inc_annotation()
        .inc_genres()
        .execute().await;

    match result {
        Ok(a) => {
            println!("{}: {}({})", a.id, a.name, a.r#type.unwrap_or_default());
            if let Some(aliases) = a.aliases {
                for alias in aliases {
                    println!("Alias: {}", alias.name);
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
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
