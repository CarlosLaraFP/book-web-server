pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self {

        }
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