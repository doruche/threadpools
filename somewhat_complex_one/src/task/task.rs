#![allow(unused)]

use std::{sync::{atomic, mpsc, Arc, Mutex}};

use crate::error::Error;

use super::{AsTask, TaskState};

pub struct TaskHandle<T> { // T
    state: Arc<atomic::AtomicU8>,
    result_receiver: Arc<Mutex<mpsc::Receiver<Result<T, Error>>>>,
    cancel_flag: Arc<atomic::AtomicBool>,
    waited: atomic::AtomicBool,
}

pub struct Task<T> { // T
    pub(crate) cancel_flag: Arc<atomic::AtomicBool>,
    pub(crate) state: Arc<atomic::AtomicU8>,
    future: Option<Box<dyn FnOnce() -> T + Send>>,
    pub(crate) result_sender: Option<mpsc::Sender<Result<T, Error>>>, // None for unstarted
}

pub trait ToTask<T> {
    fn to_task(self) -> Option<Task<T>>;
}


impl<T: Send + 'static> AsTask for Task<T> {
    fn run(self: Box<Self>) {
        if self.cancel_flag.load(atomic::Ordering::Relaxed) {
            self.result_sender.unwrap().send(Err(Error::Cancelled));
            return;
        }

        self.transition_state(TaskState::Running);
        let result = (self.future.unwrap())();
        self.result_sender.unwrap().send(Ok(result)).ok();
        self.state.store(TaskState::Completed as u8, atomic::Ordering::Release);
    }
    
    fn cancel(&self) {
        self.cancel_flag.store(true, atomic::Ordering::Release);
        self.transition_state(TaskState::Cancelled);
        self.result_sender.as_ref().unwrap().send(Err(Error::Cancelled));
    }
}

impl<F, T> ToTask<T> for F
where
    F: FnOnce() -> T + Send + 'static,
    T: Send {
    fn to_task(self) -> Option<Task<T>> {
        Some(Task::<T> {
            result_sender: None,
            cancel_flag: Arc::new(atomic::AtomicBool::new(false)),
            state: Arc::new(atomic::AtomicU8::new(TaskState::Pending as u8)),
            future: Some(Box::new(self)),
        })
    }
}

impl<T> ToTask<T> for Task<T> {
    fn to_task(self) -> Option<Task<T>> {
        Some(self)
    }
}


impl<T> Task<T> {
    pub(crate) fn new(f: impl ToTask<T>) -> Self {
        match f.to_task() {
            Some(task) => task,
            None => unreachable!(),
        }
    }

    pub(crate) fn transition_state(&self, new_state: TaskState) {
        self.state.store(new_state as u8, atomic::Ordering::Release);
    }

}

impl<T> TaskHandle<T> {
    pub fn new(task: &Task<T>, result_receiver: mpsc::Receiver<Result<T, Error>>) -> Self {
        Self {
            cancel_flag: task.cancel_flag.clone(),
            state: task.state.clone(),
            waited: atomic::AtomicBool::new(false),
            result_receiver: Arc::new(Mutex::new(result_receiver)),
        }
    }

    pub fn has_finished(&self) -> bool {
        self.state.load(atomic::Ordering::Acquire) == TaskState::Completed as u8
    }

    pub fn state(&self) -> TaskState {
        match self.state.load(atomic::Ordering::Acquire) {
            0 => TaskState::Pending,
            1 => TaskState::Running,
            2 => TaskState::Completed,
            3 => TaskState::Cancelled,
            _ => unreachable!(),
        }
    }

    pub fn wait(&self) -> Result<T, Error> {
        if self.waited.load(atomic::Ordering::Relaxed) {
            return Err(Error::MultipleWaits);
        }

        self.waited.store(true, atomic::Ordering::Relaxed);
        if self.cancel_flag.load(atomic::Ordering::Acquire) {
            return Err(Error::Cancelled);            
        }
        match self.result_receiver.lock().unwrap().recv() {
            r@Ok(..) => r.unwrap(),
            Err(e) => {
                match self.state.load(atomic::Ordering::Acquire) {
                    3 => Err(Error::Cancelled),
                    _ => Err(Error::ChannelDisconnected),
                }
            }
        }
    }

    /// If a task has already been running, then this method can't really cancel it.
    /// For such operations, you should implement the mechanism by your own.
    pub fn cancel(&self) -> Result<(), Error> {
        if self.state.load(atomic::Ordering::Acquire) != TaskState::Running as u8 {
            self.cancel_flag.store(true, atomic::Ordering::Release);
            self.state.store(TaskState::Cancelled as u8, atomic::Ordering::Release);
            Ok(())
        } else {
            Err(Error::CancelAfterRunning)
        }
    }
}