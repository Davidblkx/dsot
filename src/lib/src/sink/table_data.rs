use std::{fmt::Debug, sync::Arc};
use tokio::sync::watch::{self, Ref};

pub trait TableFilter {
    type Target;

    fn include(&self, target: &Self::Target) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRef<T: Debug + Clone + PartialEq + TableFilter> {
    pub offset: i64,
    pub size: i64,
    pub filter: T,
}

#[derive(Debug, Clone)]
pub struct TableRefWatch<T: Debug + Clone + PartialEq + TableFilter> {
    pub offset: watch::Receiver<i64>,
    set_offset: Arc<watch::Sender<i64>>,
    pub size: watch::Receiver<i64>,
    set_size: Arc<watch::Sender<i64>>,
    pub filter: watch::Receiver<T>,
    set_filter: Arc<watch::Sender<T>>,
    all: watch::Receiver<TableRef<T>>,
    set_all: Arc<watch::Sender<TableRef<T>>>,
}

impl<T: Debug + Clone + PartialEq + TableFilter> TableRefWatch<T> {
    pub fn new(offset: i64, size: i64, filter: T) -> Self {
        let (offset_tx, offset_rx) = watch::channel(offset);
        let (size_tx, size_rx) = watch::channel(size);
        let (filter_tx, filter_rx) = watch::channel(filter.clone());
        let (all_tx, all_rx) = watch::channel(TableRef::new(offset, size, filter));

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

    pub fn watch(&self) -> watch::Receiver<TableRef<T>> {
        self.all.clone()
    }

    pub fn set_offset(&self, offset: i64) {
        self.set_offset.send_replace(offset);
        self.set_all.send_if_modified(|prev| {
            if prev.offset != offset {
                prev.offset = offset;
                true
            } else {
                false
            }
        });
    }

    pub fn set_size(&self, size: i64) {
        self.set_size.send_replace(size);
        self.set_all.send_if_modified(|prev| {
            if prev.size != size {
                prev.size = size;
                true
            } else {
                false
            }
        });
    }

    pub fn set_filter(&self, filter: T) {
        self.set_filter.send_replace(filter.clone());
        self.set_all.send_if_modified(|prev| {
            if prev.filter != filter {
                prev.filter = filter;
                true
            } else {
                false
            }
        });
    }

    pub fn mod_filter(&self, modifier: impl FnOnce(Ref<'_, T>) -> Option<T>) {
        if let Some(new_value) = modifier(self.filter.borrow()) {
            self.set_filter(new_value);
        }
    }

    pub fn get_table_ref(&self) -> TableRef<T> {
        self.all.borrow().clone()
    }
}

impl<T: Debug + Clone + PartialEq + TableFilter> TableRef<T> {
    pub fn new(offset: i64, size: i64, filter: T) -> Self {
        Self {
            offset,
            size,
            filter,
        }
    }

    pub fn into_watch(self) -> TableRefWatch<T> {
        TableRefWatch::new(self.offset, self.size, self.filter)
    }
}
