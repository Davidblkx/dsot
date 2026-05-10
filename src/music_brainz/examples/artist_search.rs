use music_brainz::entities::Artist;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let result = Artist::search()
        .artist("Nirvana")
        .limit(5)
        .build()
        .execute().await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);
            for item in a.artists {
                println!(
                    "{}: {}({}|{})",
                    item.id,
                    item.name,
                    item.r#type.unwrap_or_default(),
                    item.disambiguation.unwrap_or("".to_string())
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
