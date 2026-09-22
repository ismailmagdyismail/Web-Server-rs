use crate::worker_thread::WorkerThread;

pub struct ThreadPool<T>
where
    T: FnOnce() + 'static + Send,
{
    worker_threads: Vec<WorkerThread<T>>,
    turn: usize,
}

impl<T> ThreadPool<T>
where
    T: FnOnce() + 'static + Send,
{
    pub fn new(size: usize) -> ThreadPool<T> {
        let mut vec = Vec::<WorkerThread<T>>::new();
        for _ in 0..size {
            vec.push(WorkerThread::new());
        }
        ThreadPool {
            worker_threads: vec,
            turn: 0,
        }
    }

    pub fn start(&mut self) {
        for thread in &mut self.worker_threads {
            thread.start();
        }
    }

    pub fn stop(self) {
        for thread in self.worker_threads {
            thread.stop_and_wait();
        }
    }

    pub fn submit_task(&mut self, task: T) {
        self.worker_threads[self.turn].submit_task(task);
        self.turn = if self.turn == self.worker_threads.len() - 1 {
            0
        } else {
            self.turn + 1
        };
    }
}

#[cfg(test)]
mod test {

    use std::sync::{Arc, Mutex};

    use crate::thread_pool::ThreadPool;

    #[test]
    fn test_pushing_tasks() {
        let mut pool = ThreadPool::new(10);
        let vec = Arc::new(Mutex::new(Vec::<String>::new()));
        pool.start();
        for _ in 0..20 {
            let clone = vec.clone();
            pool.submit_task(move || {
                (*clone.lock().unwrap()).push("task completed".to_string());
            });
        }
        pool.stop();
        assert_eq!(vec.lock().unwrap().len(), 20);
    }
}
