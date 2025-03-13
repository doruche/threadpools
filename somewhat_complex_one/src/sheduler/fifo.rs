#![allow(unused)]

use super::Scheduler;

pub struct FifoScheduler {

}

impl Scheduler for FifoScheduler {
    fn schedule(&self, task: crate::task::Task) {
        todo!()
    }

    fn next_task(&self) -> Option<crate::task::Task> {
        todo!()
    }
}

impl FifoScheduler {
    pub fn new() -> Self {
        todo!()
    }
}