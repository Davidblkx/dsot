use std::sync::Arc;

use dsot_model::{InboxStatus, InboxValue};
use tokio::sync::watch;
use uuid::Uuid;

use crate::{
    error::Result,
    repository::{DsotRepository, InboxRepository},
    sink::{TableRef, TableRefWatch},
};

#[derive(Debug, Clone, PartialEq)]
pub struct InboxFilterValue {
    pub status: Option<InboxStatus>,
}

#[derive(Debug, Clone)]
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
    pub async fn from_repository(repo: &DsotRepository) -> Result<Self> {
        let filter = TableRef {
            filter: InboxFilterValue { status: None },
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
