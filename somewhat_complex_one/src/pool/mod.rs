#![allow(unused)]

mod pool;
mod worker;
mod builder;

pub(crate) const MAX_POOL_SIZE: usize = 128;
pub(crate) const DEFAULT_POOL_SIZE: usize = 4;
pub use builder::ThreadPoolBuilder as ThreadPoolBuilder;
pub use pool::ThreadPool as ThreadPool;