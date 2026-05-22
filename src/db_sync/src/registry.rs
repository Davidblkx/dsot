use std::{collections::HashMap, sync::OnceLock};

use uuid::Uuid;

use crate::{
    database::{DsotDatabase, DsotDatabaseError, Result},
    model::JournalEntry,
};

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type ApplyJournalFn = fn(&DsotDatabase, JournalEntry) -> BoxFuture<'_, Result<Uuid>>;

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

    pub async fn apply_journal(&self, db: &DsotDatabase, journal_data: &[u8]) -> Result<Uuid> {
        let journal = JournalEntry::from_bytes(journal_data)?;
        match self.repos.get(journal.table.as_str()) {
            None => Err(DsotDatabaseError::RepositoryNotFound(journal.table)),
            Some(apply) => {
                let id = apply(db, journal).await?;
                Ok(id)
            }
        }
    }
}
