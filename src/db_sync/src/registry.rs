use std::{collections::HashMap, future::Future, sync::OnceLock};

use uuid::Uuid;

use crate::{
    database::{DsotDatabase, DsotDatabaseError, DsotDatabaseTransaction, Result},
    model::JournalEntry,
};

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type ApplyJournalFn = for<'a, 'b> fn(&'b mut DsotDatabaseTransaction<'a>, JournalEntry) -> BoxFuture<'b, Result<Uuid>>;

pub struct ApplyJournalRef {
    pub table: &'static str,
    pub apply_journal: ApplyJournalFn,
}

#[linkme::distributed_slice]
pub static APPLY_JOURNAL_REF: [ApplyJournalRef];

#[derive(Debug)]
pub struct RepositoryRegistry {
    repos: HashMap<&'static str, ApplyJournalFn>,
}

impl RepositoryRegistry {
    pub fn instance() -> &'static Self {
        static REGISTRY: OnceLock<RepositoryRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut repos = HashMap::new();

            for r in APPLY_JOURNAL_REF {
                repos.insert(r.table, r.apply_journal);
            }

            RepositoryRegistry { repos }
        })
    }

    pub async fn apply_journal_trx<'a, 'b>(
        &self,
        trx: &'b mut DsotDatabaseTransaction<'a>,
        journal_data: &[u8],
    ) -> Result<Uuid> {
        let journal = JournalEntry::from_bytes(journal_data)?;
        match self.repos.get(journal.table.as_str()) {
            None => Err(DsotDatabaseError::RepositoryNotFound(journal.table)),
            Some(apply) => {
                let id = apply(trx, journal).await?;
                Ok(id)
            }
        }
    }

    pub async fn apply_journal(&self, db: &DsotDatabase, journal_data: &[u8]) -> Result<Uuid> {
        let mut trx = db.begin_transaction().await?;
        let id = self.apply_journal_trx(&mut trx, journal_data).await?;
        trx.commit().await?;
        Ok(id)
    }

    pub async fn apply_journals(&self, db: &DsotDatabase, entries_data: &[Vec<u8>]) -> Result<Vec<Uuid>> {
        let mut trx = db.begin_transaction().await?;
        let mut ids = Vec::new();
        for data in entries_data {
            let id = self.apply_journal_trx(&mut trx, data.as_slice()).await?;
            ids.push(id);
        }
        trx.commit().await?;
        Ok(ids)
    }
}
