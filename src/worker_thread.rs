use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

pub struct WorkerThread<T>
where
    T: FnOnce() + Send + 'static,
{
    id: usize,
    tx_channel: Option<mpsc::Sender<T>>,
    thread: Option<JoinHandle<()>>,
}

impl<T> WorkerThread<T>
where
    T: FnOnce() + Send + 'static,
{
    pub fn new(id: usize) -> WorkerThread<T> {
        WorkerThread {
            id,
            tx_channel: None,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        if self.is_started() {
            return;
        }
        let (tx, rx) = mpsc::channel::<T>();
        self.tx_channel = Some(tx);
        self.thread = Some(thread::spawn(move || {
            WorkerThread::worker_thread_loop(rx);
        }));
    }

    pub fn stop_and_wait(&mut self) {
        self.tx_channel.take();
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
        println!("worker {}, is shutting down", self.id);
    }

    pub fn submit_task(&mut self, task: T) {
        if self.is_started() {
            let tx_channel = self.tx_channel.as_ref().unwrap();
            tx_channel.send(task).unwrap();
        }
    }

    fn is_started(&self) -> bool {
        self.thread.is_some()
    }

    fn worker_thread_loop(task_queue: mpsc::Receiver<T>) {
        while let Ok(task) = task_queue.recv() {
            task();
        }
    }
}

impl<T> Drop for WorkerThread<T>
where
    T: FnOnce() + Send + 'static,
{
    fn drop(&mut self) {
        self.stop_and_wait();
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::worker_thread::WorkerThread;

    #[test]
    fn test_pulling_tasks() {
        let mut worker_thread = WorkerThread::new(10);
        let arc = Arc::new(Mutex::new(0));
        let worker_clone = arc.clone();
        worker_thread.start();
        worker_thread.submit_task(move || {
            println!("incrementing from worker started");
            for _ in 0..100 {
                *worker_clone.lock().unwrap() += 1;
            }
            println!("incrementing from worker finished");
        });

        worker_thread.stop_and_wait();

        assert_eq!(*arc.lock().unwrap(), 100);
    }
}
