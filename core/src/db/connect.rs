use diesel::Connection;

pub fn connect_db() -> diesel::sqlite::SqliteConnection {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    diesel::sqlite::SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
