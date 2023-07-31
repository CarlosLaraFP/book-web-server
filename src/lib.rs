/*
    Note: If the operating system can’t create a thread because there aren’t enough system resources,
    thread::spawn will panic. That will cause our whole server to panic, even though the creation of
    some threads might succeed. For simplicity’s sake, this behavior is fine, but in a production
    thread pool implementation, you’d likely want to use std::thread::Builder and its spawn method
    that returns Result instead.
 */
use std::fmt::{Display, Formatter};
use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        // we clone the Arc to bump the reference count so the workers can share ownership of the receiver
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
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        /*
            We’re calling unwrap on send for the case that sending fails. This might happen if, for
            example, we stop all our threads from executing, meaning the receiving end has stopped
            receiving new messages. At the moment, we can’t stop our threads from executing: our
            threads continue executing as long as the pool exists. The reason we use unwrap is that
            we know the failure case won’t happen, but the compiler doesn’t know that.
         */
        self.sender.send(job).unwrap()
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            /*
                We first call lock on the receiver to acquire the mutex, and then we call unwrap to
                panic on any errors. Acquiring a lock might fail if the mutex is in a poisoned state,
                which can happen if some other thread panicked while holding the lock rather than
                releasing the lock. In this situation, calling unwrap to have this thread panic is
                the correct action to take. Feel free to change this unwrap to an expect with an
                error message that is meaningful to you.
             */
            let job = receiver
                .lock()
                .expect("Mutex poisoned: Another thread panicked while holding the lock.")
                .recv()
                .expect("The thread holding the sender has shut down.");

            println!("Worker {id} got a job; executing...");

            job();
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
