#![allow(unused)]

use crate::task::Task;

mod fifo;


pub use fifo::FifoScheduler as FifoScheduler;
pub(crate) trait Scheduler: Send + Sync {
    fn schedule(&self, task: Task);
    fn next_task(&self) -> Option<Task>;
}
