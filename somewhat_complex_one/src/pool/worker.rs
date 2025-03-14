#![allow(unused)]

use std::{sync::Arc, thread};

use crate::{sheduler::Scheduler, TaskState};

pub(super) struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, scheduler: Arc<dyn Scheduler>) -> Self {
        let thread = Some(thread::spawn(move || {
            while let Some(mut task) = scheduler.next_task() {
                task.run();
            }

            println!("Worker {} exiting.", id);
        }));

        Self {
            id,
            thread,
        }     
    }

    pub fn stop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}