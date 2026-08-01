use std::{fmt::Debug, sync::Arc};
use tokio::sync::watch;

#[derive(Debug, Clone)]
pub struct TableRef<T: Debug + Clone + PartialEq> {
    pub offset: watch::Receiver<u64>,
    set_offset: Arc<watch::Sender<u64>>,
    pub size: watch::Receiver<u64>,
    set_size: Arc<watch::Sender<u64>>,
    pub filter: watch::Receiver<T>,
    set_filter: Arc<watch::Sender<T>>,
    all: watch::Receiver<(u64, u64, T)>,
    set_all: Arc<watch::Sender<(u64, u64, T)>>,
}

impl<T: Debug + Clone + PartialEq> TableRef<T> {
    pub fn new(offset: u64, size: u64, filter: T) -> Self {
        let (offset_tx, offset_rx) = watch::channel(offset);
        let (size_tx, size_rx) = watch::channel(size);
        let (filter_tx, filter_rx) = watch::channel(filter.clone());
        let (all_tx, all_rx) = watch::channel((offset, size, filter));

        Self {
            offset: offset_rx,
            set_offset: Arc::new(offset_tx),
            size: size_rx,
            set_size: Arc::new(size_tx),
            filter: filter_rx,
            set_filter: Arc::new(filter_tx),
            all: all_rx,
            set_all: Arc::new(all_tx),
        }
    }

    pub fn watch(&self) -> watch::Receiver<(u64, u64, T)> {
        self.all.clone()
    }

    pub fn set_offset(&self, offset: u64) {
        self.set_offset.send_replace(offset);
        self.set_all.send_if_modified(|(prev, _, _)| {
            if prev != &offset {
                *prev = offset;
                true
            } else {
                false
            }
        });
    }

    pub fn set_size(&self, size: u64) {
        self.set_size.send_replace(size);
        self.set_all.send_if_modified(|(_, prev, _)| {
            if prev != &size {
                *prev = size;
                true
            } else {
                false
            }
        });
    }

    pub fn set_filter(&self, filter: T) {
        self.set_filter.send_replace(filter.clone());
        self.set_all.send_if_modified(|(_, _, prev)| {
            if prev != &filter {
                *prev = filter;
                true
            } else {
                false
            }
        });
    }
}
