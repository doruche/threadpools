#![allow(unused)]

use crate::{task::Task, AsTask};

mod fifo;


pub use fifo::FifoScheduler as FifoScheduler;
pub trait Scheduler: Send + Sync {
    fn schedule(&self, task: Box<dyn AsTask>);
    fn next_task(&self) -> Option<Box<dyn AsTask>>;
    fn terminate(&self);
}
