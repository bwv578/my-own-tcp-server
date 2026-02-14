use std::collections::VecDeque;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use uuid::Uuid;

pub struct Worker {
    handle: JoinHandle<()>,
    id: String
}

impl Worker {
    pub fn spawn(task_queue:Arc<Mutex<VecDeque<TcpStream>>>) -> Self {
        Self {
            handle: thread::spawn(move || {
                loop {
                    if let Some(task) = task_queue.lock().unwrap().pop_front() {
                        todo!("do task")
                        
                    }else{
                        sleep(Duration::from_millis(100));
                    }
                }
            }),

            id: Uuid::new_v4().to_string(),
        }
    }
}