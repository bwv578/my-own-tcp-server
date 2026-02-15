use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::server::worker::Worker;

pub struct ThreadPool {
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    workers: Vec<Worker>,
}

pub type Task = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size:u16) -> Self {
        let task_queue:Arc<Mutex<VecDeque<Task>>> = Arc::new(Mutex::new(VecDeque::new()));

        let mut workers:Vec<Worker> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            workers.push(
                Worker::spawn(task_queue.clone())
            );
        }

        Self { task_queue, workers }
    }

    pub fn push_task(&mut self, task: Task) {
        self.task_queue.lock().unwrap().push_back(task);
    }
}