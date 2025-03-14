#![allow(unused)]

use std::{collections::VecDeque, sync::{atomic, Arc, Condvar, Mutex}};

use crate::{task::Task, AsTask};

use super::Scheduler;

pub struct FifoScheduler {
    task_queue: Arc<(Mutex<VecDeque<Box<dyn AsTask>>>, Condvar)>,
    terminate_flag: atomic::AtomicBool,
}

impl FifoScheduler {
    pub fn new() -> Self {
        Self {
            task_queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
            terminate_flag: atomic::AtomicBool::new(false),
        }
    }
}

impl Scheduler for FifoScheduler {
    fn schedule(&self, task: Box<dyn AsTask>) {
        let (queue, condvar) = &*self.task_queue;   
        match queue.lock() {
            Ok(mut queue) => {
                queue.push_back(task);
                if queue.len() > 1 {
                    condvar.notify_all();
                } else {
                    condvar.notify_one();
                }
            },
            Err(_) => panic!("mutex poisoned."),
        };
    }

    fn next_task(&self) -> Option<Box<dyn AsTask>> {
        let (queue, condvar) = &*self.task_queue;
        let mut queue = queue.lock().expect("mutex poisoned.");
        
        loop {
            if self.terminate_flag.load(atomic::Ordering::Acquire) {
                break None;
            }

            match queue.pop_front() {
                Some(task) => return Some(task),
                None =>  {
                    queue = condvar.wait(queue).expect("mutex poisoned.");
                },
            }   
        }
    }
    
    fn terminate(&self) {
        self.terminate_flag.store(true, atomic::Ordering::Release);
        let condvar = &self.task_queue.1;
        condvar.notify_all();
        let remain_tasks = &*self.task_queue.0.lock().expect("mutex poisoned.");
        for task in remain_tasks {
            task.cancel();
        }
    }
}