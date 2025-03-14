#![allow(unused)]

use std::{
    collections::{HashMap, VecDeque}, 
    sync::{mpsc, Arc, Condvar, Mutex}
};

use crate::{sheduler::{FifoScheduler, Scheduler}, task::{Task, ToTask}, TaskHandle};
use super::worker::Worker;

pub struct ThreadPool {
    pub(super) scheduler: Arc<dyn Scheduler>,
    pub(super) workers: Vec<Worker>,
    //task_registry: Arc<Mutex<(HashMap<u16, Arc<Task>>, Condvar)>>,
    //task_queue: Arc<(Mutex<VecDeque<Task>>, Condvar)>,
}

impl ThreadPool {
    pub fn new() -> super::ThreadPoolBuilder {
        super::ThreadPoolBuilder::new()
    }

    pub fn commit<T: Send + 'static>(&self, task: impl ToTask<T>) -> TaskHandle<T> {
        let (sender, receiver) = mpsc::channel();
        
        let mut task = Task::new(task);
        task.result_sender = Some(sender);
        let handle = TaskHandle::new(&task, receiver);

        self.scheduler.schedule(Box::new(task));
        handle
    }

    pub fn terminate(self) {}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.scheduler.terminate();

        for worker in &mut self.workers {
            worker.stop();
        }
    }
}