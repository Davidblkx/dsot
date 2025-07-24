use super::{BinModel, HasBytes, StorageEntity, StorageHandler, StorageTransaction};
use crate::error::Result;

pub struct Storage<T: Sized + StorageHandler> {
    handler: T,
}

impl<T> Storage<T>
where
    T: Sized + StorageHandler,
{
    pub fn new(handler: T) -> Self {
        Self { handler }
    }

    pub fn get<Entity: BinModel + StorageEntity>(
        &self,
        key: impl HasBytes,
    ) -> Result<Option<Entity::Model>> {
        let bucket = self.handler.open(Entity::get_storage_name())?;
        let data = bucket.get(key.get_bytes())?;
        bucket.close()?;

        match data {
            Some(data) => {
                let model = Entity::deserialize(&data)?;
                Ok(Some(model))
            }
            None => Ok(None),
        }
    }

    pub fn set<Entity: BinModel + StorageEntity>(&self, model: &Entity) -> Result<()> {
        let mut bucket = self.handler.open(Entity::get_storage_name())?;
        let key = model.get_storage_key();
        let data = Entity::serialize(model)?;

        bucket.set(&key, &data)?;
        bucket.commit()?;

        Ok(())
    }

    pub fn delete<Entity: BinModel + StorageEntity>(&self, model: &Entity) -> Result<()> {
        let mut bucket = self.handler.open(Entity::get_storage_name())?;
        let key = model.get_storage_key();
        bucket.remove(&key)?;
        bucket.commit()?;

        Ok(())
    }

    pub fn delete_by_key<Entity: BinModel + StorageEntity>(&self, key: &[u8]) -> Result<()> {
        let mut bucket = self.handler.open(Entity::get_storage_name())?;
        bucket.remove(key)?;
        bucket.commit()?;

        Ok(())
    }
}
