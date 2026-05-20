use crate::dser::Result;
use crate::model::SyncOperation;
use uuid::Uuid;

pub trait SyncEntity {
    type Entity;

    fn get_id(&self) -> Uuid;
    fn op_create(&self) -> super::dser::Result<SyncOperation>;
    fn op_delete(&self) -> SyncOperation;
    fn op_restore(&self) -> SyncOperation;
    fn op_update(&self, prev: &Self::Entity) -> Option<SyncOperation>;
    fn from_bytes(data: &[u8]) -> Result<Self::Entity>;
    fn to_bytes(&self) -> Result<Vec<u8>>;
}
