#![allow(unused)]

use std::sync::Arc;

use crate::{pool::*, sheduler::{FifoScheduler, Scheduler}};

use super::{pool::ThreadPool, worker::Worker};

pub struct ThreadPoolBuilder {
    size: usize,
    scheduler: Option<Arc<dyn Scheduler>>,
}

impl ThreadPoolBuilder {
    pub fn new() -> Self {
        Self {
            size: DEFAULT_POOL_SIZE,
            scheduler: None,
        }
    }

    pub fn num_threads(self, size: usize) -> Self {
        Self {
            size,
            ..self
        }
    }

    pub fn scheduler<S>(self, scheduler: S) -> Self
    where S: Scheduler + 'static {
        Self {
            scheduler: Some(Arc::new(scheduler)),
            ..self
        }
    }

    pub fn build(mut self) -> Result<ThreadPool, ()> {
        if self.size > MAX_POOL_SIZE {
            return Err(());
        }

        if self.scheduler.is_none() {
            self.scheduler = Some(Arc::new(FifoScheduler::new()));
        }

        let mut workers = Vec::with_capacity(self.size + 7);
        (0..self.size).for_each(|id| workers.push(Worker::new(id, self.scheduler.as_ref().unwrap().clone())));

        Ok(ThreadPool {
            scheduler: self.scheduler.unwrap(),
            workers,
        })
    }
}