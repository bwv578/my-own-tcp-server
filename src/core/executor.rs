use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use crate::core::runtime::Task;

pub struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn spawn(task_queue:Arc<Mutex<VecDeque<Task>>>) -> Self {
        Self {
            handle: thread::spawn(move || {
                loop {
                    let task:Option<Task> = match task_queue.lock() {
                        Ok(mut task_queue_mutex) => task_queue_mutex.pop_front(),
                        Err(_) => None
                    };
                    match task {
                        Some(task_todo) => {
                            match catch_unwind(AssertUnwindSafe(task_todo)) {
                                Ok(Ok(_)) => {},
                                Ok(Err(_e)) => {
                                    /* TODO LOG ERROR */
                                    continue;
                                }
                                Err(_panic) => {
                                    /* TODO LOG PANIC */
                                    continue;
                                }
                            }
                        },
                        None => {sleep(Duration::from_millis(100));}
                    }
                }
            }),
        }
    }
}


pub struct ThreadPool {
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    workers: Vec<Worker>,
}

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

    pub fn push_task(&self, task: Task) -> Result<(), Box<dyn Error + '_>> {
        match self.task_queue.lock() {
            Ok(mut queue) => {
                Ok(queue.push_back(task))
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}