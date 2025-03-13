#![allow(unused)]

pub struct TaskHandle { // T

}

pub(crate) struct Task { // T
    job: Box<dyn FnOnce() + Send + 'static>
}

pub trait ToTask {
    fn to_task(self) -> Option<Task>;
}

impl<F> ToTask for F
where F: FnOnce() + Send + 'static {
    fn to_task(self) -> Option<Task> {
        Some(Task {
            job: Box::new(self),
        })
    }
}


impl Task {
    pub fn new(f: impl ToTask) -> Self {
        match f.to_task() {
            Some(task) => task,
            None => unreachable!(),
        }
    }

    pub fn run(self) {
        (self.job)();
    }
}