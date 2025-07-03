/// There are two types of databases in the runtime:
/// 1. The main database, which is used for the runtime's core functionality.
/// 2. User-specific databases, which are created for each user to store their data.
pub enum DbKind<'a> {
    Main,
    User(&'a str),
}
