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

        match self.apply_from_id(&mut trx, first_id).await {
            Ok(_) => {
                trx.commit().await?;
                Ok(ids)
            }
            Err(e) => {
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
        for jrn in entries {
            let JournalEntry { table, op, .. } = JournalEntry::from_bytes(jrn.as_slice())?;

            if let Some(apply) = self.repos.get(table.as_str()) {
                apply(trx, op).await?;
            } else {
                return Err(DsotDatabaseError::RepositoryNotFound(table));
            }
        }

        Ok(())
    }
}
