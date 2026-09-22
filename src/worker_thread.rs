use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

type ThreadSafeFlag = Arc<Mutex<bool>>;
pub struct WorkerThread<T>
where
    T: FnOnce(),
{
    is_running: ThreadSafeFlag,
    tx_channel: Option<mpsc::Sender<T>>,
    thread: Option<JoinHandle<()>>,
}

impl<T> WorkerThread<T>
where
    T: FnOnce() + Send + Sync + 'static,
{
    pub fn new() -> WorkerThread<T> {
        WorkerThread {
            is_running: Arc::new(Mutex::new(false)),
            tx_channel: None,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let flag_clone: Arc<Mutex<bool>> = self.is_running.clone();
        let (tx, rx) = mpsc::channel::<T>();
        self.tx_channel = Some(tx);
        self.thread = Some(thread::spawn(move || {
            WorkerThread::worker_thread_loop(flag_clone, rx);
        }));
    }

    pub fn stop_and_wait(self) {
        {
            *self.is_running.lock().unwrap() = false;
            let channel_to_close = self.tx_channel.unwrap();
            drop(channel_to_close);
        }
        self.thread.unwrap().join().unwrap();
    }

    pub fn submit_task(&mut self, task: T) {
        if *self.is_running.lock().unwrap() {
            let tx_channel = self.tx_channel.as_ref().unwrap();
            tx_channel.send(task).unwrap();
        }
    }

    fn worker_thread_loop(running_flag: ThreadSafeFlag, task_queue: mpsc::Receiver<T>) {
        loop {
            {
                let is_running = *running_flag.lock().unwrap();
                if !is_running {
                    println!("stooped ");
                    break;
                }
            }
            if let Ok(task) = task_queue.recv() {
                task();
            } else {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    use crate::worker_thread::WorkerThread;

    #[test]
    fn test_pulling_tasks() {
        let mut worker_thread = WorkerThread::new();
        let arc = Arc::new(Mutex::new(1));
        let worker_clone = arc.clone();
        worker_thread.start();
        worker_thread.submit_task(move || {
            println!("incrementing from worker started");
            *worker_clone.lock().unwrap() += 1;
            println!("incrementing from worker finished");
        });

        thread::sleep(Duration::from_millis(10));
        worker_thread.stop_and_wait();

        assert_eq!(*arc.lock().unwrap(), 2);
    }
}
