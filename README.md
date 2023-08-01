# Multithreaded Web Server

Rust Book final project: Basic web server that uses a thread pool to respond asynchronously, including graceful shutdown of the server, which cleans up all the threads in the pool.

Ideas to continue enhancing this project:

Add more documentation to ThreadPool and its public methods.
Add tests of the library’s functionality.
Change calls to unwrap to more robust error handling.
Use ThreadPool to perform some task other than serving web requests.
Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.