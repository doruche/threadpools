#![allow(unused)]

use std::{sync::Arc, thread};

use crate::sheduler::Scheduler;

pub(super) struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, scheduler: Arc<dyn Scheduler>) -> Self {
        let thread = Some(thread::spawn(move || loop {
            if let Some(task) = scheduler.next_task() {
                task.run();
            } else {
                thread::yield_now();
            }
        }));

        Self {
            id,
            thread,
        }     
    }

    fn stop(&self) {
        todo!()
    }
}