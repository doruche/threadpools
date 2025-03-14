#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Pending = 0,
    Running = 1,
    Completed = 2,
    Cancelled = 3,
}