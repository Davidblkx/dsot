use music_brainz::entities::Work;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("example_mb", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let search = Work::search()
        .work("The Sound of Silence")
        .and()
        .artist("Simon & Garfunkel")
        .limit(5)
        .build();

    println!("search: {:?}", search.get_url().unwrap().to_string());

    let result = search
        .execute()
        .await;

    match result {
        Ok(a) => {
            println!("count: {}", a.count);

            for w in a.works {
                println!("title: {}", w.title);
                match w.relations {
                    Some(relations) => {
                        for r in relations {
                            if let Some(a) = r.artist {
                                println!("artist: {}", a.name);
                            }
                        }
                    }
                    None => println!("No relations found"),
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
