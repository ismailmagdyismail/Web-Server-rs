use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

//- Running worker thread, started once created
//- Simplifies state machine
type ThreadSafeFlag = Arc<Mutex<bool>>;
pub struct RunningWorkerThread<T>
where
    T: FnOnce(),
{
    tx_channel: mpsc::Sender<T>,
    thread: JoinHandle<()>,
    is_stopped: ThreadSafeFlag,
}

impl<T> RunningWorkerThread<T>
where
    T: FnOnce() + Send + Sync + 'static,
{
    pub fn new() -> RunningWorkerThread<T> {
        let (tx, rx) = mpsc::channel::<T>();
        let flag = Arc::new(Mutex::new(false));
        let flag_clone = flag.clone();
        RunningWorkerThread {
            tx_channel: tx,
            is_stopped: flag,
            thread: thread::spawn(move || {
                RunningWorkerThread::worker_loop(flag_clone, rx);
            }),
        }
    }

    pub fn stop_and_wait(self) {
        {
            *self.is_stopped.lock().unwrap() = true;
            drop(self.tx_channel);
        }
        self.thread.join().unwrap();
    }

    fn worker_loop(stopped_flag: ThreadSafeFlag, task_queue: mpsc::Receiver<T>) {
        loop {
            {
                let is_stopped = *stopped_flag.lock().unwrap();
                if is_stopped {
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
