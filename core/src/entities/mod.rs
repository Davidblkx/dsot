#[macro_use]
pub mod utils;

pub mod files;
pub mod artist;
pub mod tag_type;
pub mod tag;
pub mod album;
pub mod release;

pub use files::File;
pub use artist::Artist;
pub use tag_type::TagType;
pub use tag::Tag;
pub use album::Album;
pub use release::Release;
