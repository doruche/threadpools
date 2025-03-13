#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Cancelled,
}