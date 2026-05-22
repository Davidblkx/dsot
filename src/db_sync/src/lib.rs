#![allow(async_fn_in_trait)]

pub mod db;
pub mod dser;
pub mod entity;
pub mod model;
pub mod repo;

pub use entity::{IntoSyncEntity, SyncEntity};
pub use repo::SyncEntityRepository;
