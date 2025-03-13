#![allow(unused)]

use std::{
    collections::{HashMap, VecDeque}, 
    sync::{Arc, Condvar, Mutex}
};

use crate::{sheduler::{FifoScheduler, Scheduler}, task::{Task, ToTask}};
use super::worker::Worker;



pub struct ThreadPool {
    pub(crate) scheduler: Arc<dyn Scheduler>,
    pub(crate) workers: Vec<Worker>,
    //task_registry: Arc<Mutex<(HashMap<u16, Arc<Task>>, Condvar)>>,
    //task_queue: Arc<(Mutex<VecDeque<Task>>, Condvar)>,
}


impl ThreadPool {
    pub fn commit(&self, task: impl ToTask) {
        let task = Task::new(task);
        self.scheduler.schedule(task);
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        todo!()
    }
}

