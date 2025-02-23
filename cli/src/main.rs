use music_brainz::entities::artist::Artist;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("dsot", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let search = Artist::search().artist("Pink Floyd").build();
    let res = search.execute().await;

    match res {
        Ok(a) => {
            println!("count: {}", a.count);
            for artist in a.artists {
                println!("{}: {}", artist.id, artist.name);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
