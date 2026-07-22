use std::collections::HashMap;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

pub type JobId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Canceled,
    Completed,
    Message(String),
    Progress {
        message: Option<String>,
        completed: u32,
        total: u32,
    },
    Initializing,
}

impl JobStatus {
    pub fn is_done(&self) -> bool {
        match self {
            JobStatus::Canceled => true,
            JobStatus::Completed => true,
            _ => false,
        }
    }

    pub fn is_canceled(&self) -> bool {
        matches!(self, JobStatus::Canceled)
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, JobStatus::Completed)
    }
}

#[derive(Debug, Clone)]
pub struct JobHandle {
    pub id: JobId,
    name: Arc<str>,
    cancel_token: CancellationToken,
    pub status: watch::Receiver<JobStatus>,
    status_sender: Arc<watch::Sender<JobStatus>>,
}

impl JobHandle {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_done(&self) -> bool {
        self.cancel_token.is_cancelled() || self.status.borrow().is_done()
    }

    pub fn is_canceled(&self) -> bool {
        self.cancel_token.is_cancelled() || self.status.borrow().is_canceled()
    }

    pub fn is_completed(&self) -> bool {
        self.status.borrow().is_completed()
    }

    pub fn cancel(&self) {
        self.cancel_token.cancel();
        self.update_status(JobStatus::Canceled);
    }

    pub fn complete(&self) {
        self.update_status(JobStatus::Completed);
    }

    pub fn report_progress(&self, message: Option<String>, completed: u32, total: u32) {
        self.update_status(JobStatus::Progress {
            message,
            completed,
            total,
        });
    }

    pub fn report_message(&self, message: impl ToString) {
        self.update_status(JobStatus::Message(message.to_string()));
    }

    pub fn update_status(&self, status: JobStatus) {
        if self.status.borrow().is_done() {
            return;
        }

        self.status_sender.send_if_modified(|s| {
            if s == &status {
                false
            } else {
                *s = status;
                true
            }
        });
    }

    fn new(id: u64, name: &str) -> Self {
        let (status_sender, status) = watch::channel(JobStatus::Initializing);
        let cancel_token = CancellationToken::new();
        Self {
            id,
            name: Arc::from(name),
            cancel_token,
            status,
            status_sender: Arc::new(status_sender),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobManager {
    jobs: Arc<RwLock<HashMap<JobId, JobHandle>>>,
    next_id: Arc<AtomicU64>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::with_capacity(1000))),
            next_id: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Creates a new background job
    pub fn create(&self, name: &str) -> JobHandle {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let job = JobHandle::new(id, name);

        let mut writer = self.jobs.write().unwrap();
        writer.insert(id, job.clone());

        job
    }

    /// Cancel and remove a job
    pub fn remove(&self, id: JobId) -> Option<JobHandle> {
        if let Some(job) = self.jobs.write().unwrap().remove(&id) {
            job.cancel();
            Some(job)
        } else {
            None
        }
    }

    /// Fetches a active job by ID.
    pub fn get(&self, id: JobId) -> Option<JobHandle> {
        self.jobs.read().unwrap().get(&id).cloned()
    }

    /// Returns all currently tracked active jobs for UI listings.
    pub fn active_jobs(&self) -> Vec<JobHandle> {
        self.jobs.read().unwrap().values().cloned().collect()
    }
}
