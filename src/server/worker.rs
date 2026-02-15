use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use uuid::Uuid;
use crate::server::thread_pool::Task;

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
                        task_queue.lock().unwrap().pop_front()
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