use std::{collections::HashMap, future::Future, sync::OnceLock};

use uuid::Uuid;

use crate::{
    database::{DsotDatabase, DsotDatabaseError, DsotDatabaseTransaction, Result},
    model::{JournalEntry, SyncOperation},
};

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type ApplySqlOperation =
    for<'a, 'b> fn(&'b mut DsotDatabaseTransaction<'a>, SyncOperation) -> BoxFuture<'b, Result<()>>;

pub struct ApplySqlOperationRef {
    pub table: &'static str,
    pub apply: ApplySqlOperation,
}

#[linkme::distributed_slice]
pub static APPLY_SQL_OPERATION_REF: [ApplySqlOperationRef];

#[derive(Debug)]
pub struct RepositoryRegistry {
    repos: HashMap<&'static str, ApplySqlOperation>,
}

impl RepositoryRegistry {
    pub fn instance() -> &'static Self {
        static REGISTRY: OnceLock<RepositoryRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut repos = HashMap::new();

            for r in APPLY_SQL_OPERATION_REF {
                log::trace!("Registering repository for table '{}'", r.table);
                repos.insert(r.table, r.apply);
            }

            RepositoryRegistry { repos }
        })
    }

    /// Insert journals if new, and reconstruct sql database
    pub async fn apply_journals_bytes(
        &self,
        db: &DsotDatabase,
        entries: &[&[u8]],
    ) -> Result<Vec<Uuid>> {
        if entries.len() == 0 {
            return Ok(Vec::new());
        }

        let mut journals = Vec::new();
        for jrn_bytes in entries {
            journals.push(JournalEntry::from_bytes(jrn_bytes)?);
        }

        let ids = self.apply_journals(db, journals).await?;
        log::trace!("Applied {} journal entries", ids.len());
        Ok(ids)
    }

    /// Insert journals if new, and reconstruct sql database
    pub async fn apply_journals(
        &self,
        db: &DsotDatabase,
        entries: Vec<JournalEntry>,
    ) -> Result<Vec<Uuid>> {
        if entries.len() == 0 {
            return Ok(Vec::new());
        }

        log::debug!("apply_journals: {} candidate entries", entries.len());

        let mut trx = db.begin_transaction().await?;
        let mut ids = Vec::new();
        let mut first_id = Uuid::max();

        for journal in entries {
            let id = journal.id;
            if trx.insert_journal(journal)? {
                ids.push(id);
                if id < first_id {
                    first_id = id;
                }
            }
        }

        log::debug!("apply_journals: {} new entries; replaying from {}", ids.len(), first_id);

        match self.apply_from_id(&mut trx, first_id).await {
            Ok(_) => {
                trx.commit().await?;
                Ok(ids)
            }
            Err(e) => {
                log::warn!("apply_journals failed during replay: {}; rolling back", e);
                trx.rollback().await?;
                Err(e)
            }
        }
    }

    async fn apply_from_id<'a, 'b>(
        &self,
        trx: &'b mut DsotDatabaseTransaction<'a>,
        id: Uuid,
    ) -> Result<()> {
        let entries = trx.get_entries_since(id.as_bytes())?;
        log::trace!("apply_from_id: replaying {} entries from {}", entries.len(), id);
        for jrn in entries {
            let JournalEntry { table, op, .. } = JournalEntry::from_bytes(jrn.as_slice())?;

            if let Some(apply) = self.repos.get(table.as_str()) {
                log::trace!("dispatching journal op to '{}'", table);
                apply(trx, op).await?;
            } else {
                log::error!("no repository registered for table '{}'", table);
                return Err(DsotDatabaseError::RepositoryNotFound(table));
            }
        }

        Ok(())
    }
}
