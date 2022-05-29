use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct WorkerPool
{
    workers: Vec<Worker>,
    worker_sender: mpsc::Sender<WorkerMessage>
}

pub struct Worker
{
    id: usize,
    thread_handle: Option<thread::JoinHandle<()>>
}

pub enum WorkerMessage
{
    NewJob(WorkerJob),
    Terminate
}

type WorkerJob = Box<dyn FnOnce() + Send + 'static>;

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WorkerMessage>>>) -> Worker
    {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv().unwrap() {
                WorkerMessage::NewJob(job) => {
                    println!("Worker {} has received a new job to process", id);
                    job();
                },
                WorkerMessage::Terminate => {
                    println!("Worker {} has been terminated", id);
                    break;
                }
            }
        });

        Worker
        {
            id,
            thread_handle: Some(thread)
        }
    }
}

impl WorkerPool
{
    pub fn new(n_threads: usize) -> WorkerPool
    {
        let (channel_sender, channel_received): (Sender<WorkerMessage>, Receiver<WorkerMessage>) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(channel_received));

        let mut workers = Vec::with_capacity(n_threads);

        for id in 0..n_threads
        {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        WorkerPool
        {
            workers,
            worker_sender: channel_sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.worker_sender.send(WorkerMessage::NewJob(job)).unwrap();
    }
}

impl Drop for WorkerPool
{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.worker_sender.send(WorkerMessage::Terminate).unwrap();
        }

        for worker in &mut self.workers
        {
            if let Some(thread) = worker.thread_handle.take()
            {
                thread.join().unwrap();
            }
        }
    }
}