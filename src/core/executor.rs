use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use crate::core::runtime::{Queue, Task};

pub struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn spawn<Q:Queue>(task_queue:Arc<Q>) -> Self {
        Self {
            handle: thread::spawn(move || {
                loop {
                    let task:Option<Task> = task_queue.pop();
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

pub struct ThreadPool<Q>
where
    Q: Queue + Send + Sync + 'static,
{
    //task_queue: Arc<Mutex<VecDeque<Task>>>,
    task_queue: Arc<Q>,
    workers: Vec<Worker>,
}

impl Queue for Mutex<VecDeque<Task>> {
    fn push(&self, item: Task) {
        self.lock().unwrap().push_back(item);
    }
    fn pop(&self) -> Option<Task> {
        self.lock().unwrap().pop_front()
    }
}

impl<Q:Queue> ThreadPool<Q> {

    pub fn new(size:u16, queue:Q) -> Self {
        let task_queue:Arc<Q> = Arc::new(queue);
        let mut workers:Vec<Worker> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            workers.push(
                Worker::spawn(task_queue.clone())
            );
        }

        Self { task_queue, workers }
    }

    pub fn push_task(&self, task: Task) {
        self.task_queue.push(task)
    }
}