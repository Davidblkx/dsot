use music_brainz::search::entities::artist::ArtistQuery;
use music_brainz::search::SearchQuery;

#[tokio::main]
async fn main() {
    env_logger::init();

    let query = ArtistQuery::for_query("Nirvana");
    let res = query.execute().await;

    match res {
        Ok(json) => {
            println!("{}", json);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
