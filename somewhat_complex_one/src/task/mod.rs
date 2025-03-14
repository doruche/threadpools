#![allow(unused)]

mod task;
mod state;

pub use task::ToTask;
pub use task::{Task, TaskHandle};
pub use state::TaskState::{self, *};
pub trait AsTask: Send {
    fn run(self: Box<Self>);
    fn cancel(&self);
}
