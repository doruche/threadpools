#![allow(unused)]

use std::collections::VecDeque;
use std::sync::{atomic, Arc, Condvar, Mutex};
use std::time::Duration;
use std::{process, thread};

pub struct ThreadPool {
    executing: Arc<atomic::AtomicBool>,
    size: usize,
    workers: VecDeque<Option<Worker>>,
    jobs: Arc<Mutex<VecDeque<Job>>>,
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

struct Job {
    task: Task,
}

impl Job {
    pub fn new(task: Task) -> Self {
        Self { task }
    }

    fn run(self) {
        (self.task)();
    }
}

impl Worker {
    fn new(pool_jobs: Arc<Mutex<VecDeque<Job>>>, pool_alive: Arc<atomic::AtomicBool>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = {
                let mut pool_jobs = pool_jobs.lock().expect("mutex poisoned.");
                if !pool_alive.load(atomic::Ordering::Relaxed)
                && pool_jobs.is_empty() {
                    break;
                }
                pool_jobs.pop_front()
            };
            if let Some(job) = job {
                job.run();
            } else {
                thread::sleep(Duration::from_micros(0));
            }
            });
        Self {
            thread,
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let executing = Arc::new(atomic::AtomicBool::new(true));

        let jobs = Arc::new(Mutex::new(VecDeque::new()));

        let mut workers = VecDeque::with_capacity(size);
        for _ in 0..size {
            workers.push_back(Some(Worker::new(jobs.clone(), executing.clone())));
        }

        Self {
            executing,
            size,
            workers,
            jobs,
        }
    }

    pub fn execute<F>(&self, task: F)
    where F : FnOnce() + Send + 'static {
        match self.jobs.lock() {
            Ok(mut jobs) => {
                jobs.push_back(Job::new(Box::new(task)));
            },
            Err(_) => panic!("mutex poisoned."),
        };
    }

    pub fn terminate(self) {}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.executing.store(false, atomic::Ordering::Relaxed);

        let mut etask = vec![];
        loop {
            let job = { self.jobs.lock().expect("mutex poisoned.").pop_front() };
            match job {
                Some(task) => etask.push(thread::spawn(move || task.run())),
                None => break,
            };
        }
        for task in etask {
            task.join();
        }

        for worker in &mut self.workers {
            match worker.take().unwrap().thread.join() {
                Ok(()) => (),
                Err(_) => {
                    eprintln!("some thread has already exited with panic");
                }
            }
        }
    }
}