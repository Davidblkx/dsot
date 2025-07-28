use redb::{Database, MultimapTableHandle, TableHandle};

fn main() {
    let db = Database::open("C:\\Users\\dpires\\.dsot\\library.db.journal").expect("failled");

    let read = db.begin_read().expect("read");

    let count = read.list_tables().iter().count();
    println!("tables: {}", count);
    for t in read.list_tables().expect("list") {
        println!("table: {}", t.name());
    }

    let count = read.list_multimap_tables().iter().count();
    println!("tables: {}", count);
    for t in read.list_multimap_tables().expect("list") {
        println!("table: {}", t.name());
    }
}
