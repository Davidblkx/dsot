use std::{fmt::Display, sync::Arc};

use dsot_model::{InboxStatus, InboxValue};
use tokio::sync::watch;
use uuid::Uuid;

use crate::{
    error::Result,
    repository::{DsotRepository, InboxRepository},
    sink::{TableFilter, TableRef, TableRefWatch},
};

#[derive(Debug, Clone, PartialEq)]
pub struct InboxFilterValue {
    pub status: Option<InboxStatus>,
}

impl TableFilter for InboxFilterValue {
    type Target = InboxItemValue;

    fn include(&self, target: &Self::Target) -> bool {
        if let Some(status) = &self.status {
            &target.status == status
        } else {
            true
        }
    }
}

impl Display for InboxFilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InboxFilterValue(status={:?})", self.status)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InboxItemValue {
    pub id: Uuid,
    pub value: InboxValue,
    pub status: InboxStatus,
}

#[derive(Debug, Clone)]
pub struct InboxState {
    pub table: TableRefWatch<InboxFilterValue>,
    pub items: watch::Receiver<Vec<InboxItemValue>>,
    writer: Arc<watch::Sender<Vec<InboxItemValue>>>,
}

pub type InboxFilter = TableRef<InboxFilterValue>;

impl InboxState {
    pub fn set_items(&self, items: Vec<InboxItemValue>) {
        self.writer.send_replace(items);
    }

    pub async fn from_repository(repo: &DsotRepository) -> Result<Self> {
        let filter = TableRef {
            filter: InboxFilterValue {
                status: Some(InboxStatus::Pending),
            },
            offset: 0,
            size: 10,
        };

        let repository = repo.clone();
        let initial = repository.load_inbox(&filter).await?;

        let table = filter.into_watch();
        let (writer, items) = watch::channel(initial);

        let state = Self {
            table,
            items,
            writer: Arc::new(writer),
        };

        let mut watcher = state.table.watch();
        let writer_ref = Arc::clone(&state.writer);

        tokio::spawn(async move {
            while watcher.changed().await.is_ok() {
                let filter = watcher.borrow().clone();
                match repository.load_inbox(&filter).await {
                    Ok(items) => {
                        if *watcher.borrow() == filter {
                            writer_ref.send_replace(items);
                        }
                    }
                    Err(err) => {
                        ::log::error!("Failed to load inbox: {}", err);
                    }
                }
            }
        });

        Ok(state)
    }
}
