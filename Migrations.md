# How to

install sqlx-cli
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

set the environment variable
```bash
$env.DATABASE_URL = "sqlite://./target/my_db.db3"
```

create a new database
```bash
sqlx database create
```

create a new migration
```bash
sqlx migrate add <name>
```

or run the migrations
```bash
sqlx migrate run
```
