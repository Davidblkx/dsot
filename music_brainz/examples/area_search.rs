use music_brainz::entities::Area;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let query = Area::search()
        .area("Portugal")
        .limit(5)
        .build();

    let result = query.execute().await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);
            for area in a.areas {
                println!("{}: {}({})", area.id, area.name, area.r#type);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
