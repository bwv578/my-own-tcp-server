use std::collections::VecDeque;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::server::worker::Worker;

pub struct ThreadPool {
    task_queue: Arc<Mutex<VecDeque<TcpStream>>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size:u16) -> Self {
        let task_queue = Arc::new(Mutex::new(VecDeque::new()));

        let mut workers:Vec<Worker> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            workers.push(
                Worker::spawn(task_queue.clone())
            );
        }

        Self {
            task_queue,
            workers
        }
    }

}