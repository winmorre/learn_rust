use std::{
    sync::{mpsc,Arc,Mutex},
    thread,
};

pub struct ThreadPool{
   workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool{
    pub fn new(size:usize)->ThreadPool{
        assert!(size  > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender,receiver) = mpsc::channel();
        receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)))
        }
        ThreadPool { workers,sender: Some(sender),}
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers{
            println!("Shutting down worker {}",worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker{
        let thread = thread::spawn(move || {
            while let Ok(job)= receiver.lock().unwrap().recv().unwrap() {
                println!("Worker {id} got a job; executing.");
                job();
            }
        });

        Worker {id, thread:mpsc: Some(thread)}
    }
}

struct Job;
