use music_brainz::entities::artist::ArtistQueryBuilder;

#[tokio::main]
async fn main() {
    env_logger::init();

    let query = ArtistQueryBuilder::new().name("Pink Floyd").build();
    let res = query.execute().await;

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
