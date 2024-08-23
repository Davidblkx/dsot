#[macro_use]
pub mod utils;

pub mod files;
pub mod artist;
pub mod tag_type;
pub mod tag;
pub mod album;
pub mod release;
pub mod track;
pub mod rel;

pub use album::Album;
pub use artist::Artist;
pub use files::File;
pub use release::Release;
pub use tag_type::TagType;
pub use tag::Tag;
