/*
    Note: If the operating system can’t create a thread because there aren’t enough system resources,
    thread::spawn will panic. That will cause our whole server to panic, even though the creation of
    some threads might succeed. For simplicity’s sake, this behavior is fine, but in a production
    thread pool implementation, you’d likely want to use std::thread::Builder and its spawn method
    that returns Result instead.
 */
use std::fmt::{Display, Formatter};
use std::{sync::{mpsc, Arc, Mutex}, thread};

// cargo doc --open
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// The `build` function returns an error type if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::InvalidSize);
        }

        // Taking a job off the channel queue involves mutating the receiver,
        // so we need thread-safe smart pointers to share and modify receiver.
        let (sender, receiver) = mpsc::channel();
        // Mutex owns the receiver, Arc tracks mutex-wrapped receiver reference counts across threads
        let receiver = Arc::new(Mutex::new(receiver));

        /*
            The with_capacity function performs the same task as Vec::new but with an important
            difference: it pre-allocates space in the vector. Because we know we need to store size
            elements in the vector, doing this allocation up front is slightly more efficient than
            using Vec::new, which resizes itself as elements are inserted.
         */
        let mut workers = Vec::with_capacity(size);

        (0..size).for_each(|id|
            workers.push(
                Worker::new(
                    id,
                    Arc::clone(&receiver)
                )
            )
        );

        Ok(
            ThreadPool { workers, sender }
        )
    }

    /*
        We need Send to transfer the closure from one thread to another and
        'static because we don’t know how long the thread will take to execute.
        Use FnOnce as a bound when you want to accept a parameter of function-like type and only
        need to call it once. If you need to call the parameter repeatedly, use FnMut as a bound;
        if you also need it to not mutate state, use Fn.
     */
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
    }
}

struct Job;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(|| {
            receiver;
        });

        Self {
            id,
            thread
        }
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidSize
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PoolCreationError {}
