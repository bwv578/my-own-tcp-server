use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use uuid::Uuid;
use crate::server::server::Task;

pub struct Worker {
    handle: JoinHandle<()>,
    id: String
}

impl Worker {
    pub fn spawn(task_queue:Arc<Mutex<VecDeque<Task>>>) -> Self {
        Self {
            handle: thread::spawn(move || {
                loop {
                    let task:Option<Task> = {
                        match task_queue.lock() {
                            Ok(mut task_queue_mutex) => {
                                task_queue_mutex.pop_front()
                            },
                            Err(_) => None
                        }
                    }; // 중괄호 닫힐때 락 해제

                    if let Some(task) = task {
                        task();
                    }else{
                        sleep(Duration::from_millis(100));
                    }
                }
            }),
            id: Uuid::new_v4().to_string(),
        }
    }
}